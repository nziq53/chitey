use bytes::{Bytes, Buf};
use h3::{quic::BidiStream, server::RequestStream};
use core::pin::Pin;
use std::{task::{Context, Poll}, sync::{Mutex, Arc}};
use futures_util::{Future, Stream};

pub struct StreamWrapper<W> 
where
W: BidiStream<Bytes> + 'static + Send + Sync
{
    inner: Arc<Mutex<RequestStream<W::RecvStream, Bytes>>>,
    inner_read: Arc<Mutex<Pin<Box<dyn Future<Output = Option<Result<Bytes, h3::Error>>>>>>>,
}

impl<W> StreamWrapper<W>
where
    W: BidiStream<Bytes> + 'static + Send + Sync
{
    pub fn new(inner: RequestStream<W::RecvStream, Bytes>) -> Self {
        let inner = Arc::new(Mutex::new(inner));
        Self {
            inner: inner.clone(),
            inner_read: Arc::new(Mutex::new(Box::pin(Self::recv_data_wrap(inner)))),
        }
    }

    async fn recv_data_wrap(inner: Arc<Mutex<RequestStream<W::RecvStream, Bytes>>>) -> Option<Result<Bytes, h3::Error>>
    where
        W: BidiStream<Bytes> + 'static + Send + Sync
    {
        match inner.lock().unwrap().recv_data().await {
        Ok(v) => {
            match v {
                Some(data) => Some(Ok(Bytes::copy_from_slice(data.chunk()))),
                None => {None}
            }
        },
        Err(e) => {Some(Err(e))}
        }
    }
}

impl<W> Stream for StreamWrapper<W>
where
W: BidiStream<Bytes> + 'static + Send + Sync
{
    type Item = Result<Bytes, h3::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<Option<Self::Item>>
    {
        let mut p = self.inner_read.lock().unwrap();
        match Pin::new(&mut *p).poll(cx) {
            Poll::Ready(bytes) => {
                *p = Box::pin(Self::recv_data_wrap(self.inner.clone()));
                Poll::Ready(bytes)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

unsafe impl<W> Send for StreamWrapper<W>
where
W: BidiStream<Bytes> + 'static + Send + Sync { }
