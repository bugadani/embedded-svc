use core::fmt::{Debug, Display};
use core::result::Result;
use core::time::Duration;

use crate::service::Service;

pub trait Spin: Service {
    fn spin(&mut self, duration: Option<Duration>) -> Result<(), Self::Error>;
}

pub trait Postbox<P>: Service {
    fn post(&mut self, payload: P, wait: Option<Duration>) -> Result<bool, Self::Error>;
}

pub trait EventBus<P>: Service {
    type Subscription;

    type Postbox: Postbox<P>;

    fn subscribe<E>(
        &mut self,
        callback: impl for<'a> FnMut(&'a P) -> Result<(), E> + Send + 'static,
    ) -> Result<Self::Subscription, Self::Error>
    where
        E: Display + Debug + Send + Sync + 'static;

    fn postbox(&mut self) -> Result<Self::Postbox, Self::Error>;
}

pub trait PinnedEventBus<P>: Service {
    type Subscription;

    type Postbox: Postbox<P>;

    fn subscribe<E>(
        &mut self,
        callback: impl for<'a> FnMut(&'a P) -> Result<(), E> + 'static,
    ) -> Result<Self::Subscription, Self::Error>
    where
        E: Display + Debug + Send + Sync + 'static;

    fn postbox(&mut self) -> Result<Self::Postbox, Self::Error>;
}

pub mod nonblocking {
    use core::fmt::{Debug, Display};
    use core::future::Future;

    use crate::service::Service;

    pub use super::Spin;

    pub trait Postbox<P>: Service {
        type PostFuture<'a>: Future<Output = Result<(), Self::Error>>
        where
            Self: 'a;

        fn post(&mut self, payload: P) -> Self::PostFuture<'_>;
    }

    /// core.stream.Stream is not stable yet. Therefore, we have to use a Future instead
    pub trait Subscription<P>: Service {
        type NextFuture<'a>: Future<Output = Result<P, Self::Error>>
        where
            Self: 'a;

        fn next(&mut self) -> Self::NextFuture<'_>;
    }

    pub trait EventBus<P>: Service {
        type Subscription: Subscription<P>;

        type Postbox: Postbox<P>;

        fn subscribe<E>(&mut self) -> Result<Self::Subscription, Self::Error>
        where
            E: Display + Debug + Send + Sync + 'static;

        fn postbox(&mut self) -> Result<Self::Postbox, Self::Error>;
    }
}
