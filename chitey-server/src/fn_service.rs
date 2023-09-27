use core::task;
use std::{marker::PhantomData, task::Poll};

use futures_util::Future;

pub fn fn_service<F, Fut, Req, Res, Err, Cfg>(
  f: F,
) -> FnServiceFactory<F, Fut, Req, Res, Err, Cfg>
where
  F: Fn(Req) -> Fut + Clone,
  Fut: Future<Output = Result<Res, Err>>,
{
  FnServiceFactory::new(f)
}

pub struct FnServiceFactory<F, Fut, Req, Res, Err, Cfg>
where
    F: Fn(Req) -> Fut,
    Fut: Future<Output = Result<Res, Err>>,
{
    f: F,
    _t: PhantomData<fn(Req, Cfg)>,
}

impl<F, Fut, Req, Res, Err, Cfg> FnServiceFactory<F, Fut, Req, Res, Err, Cfg>
where
    F: Fn(Req) -> Fut + Clone,
    Fut: Future<Output = Result<Res, Err>>,
{
    fn new(f: F) -> Self {
        FnServiceFactory { f, _t: PhantomData }
    }
}

impl<F, Fut, Req, Res, Err, Cfg> Clone for FnServiceFactory<F, Fut, Req, Res, Err, Cfg>
where
    F: Fn(Req) -> Fut + Clone,
    Fut: Future<Output = Result<Res, Err>>,
{
    fn clone(&self) -> Self {
        Self::new(self.f.clone())
    }
}

