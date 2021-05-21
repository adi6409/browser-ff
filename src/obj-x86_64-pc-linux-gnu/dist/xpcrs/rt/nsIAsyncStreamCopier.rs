//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIAsyncStreamCopier.idl
//


/// `interface nsIAsyncStreamCopier : nsIRequest`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAsyncStreamCopier {
    vtable: *const nsIAsyncStreamCopierVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAsyncStreamCopier.
unsafe impl XpCom for nsIAsyncStreamCopier {
    const IID: nsIID = nsID(0x5a19ca27, 0xe041, 0x4aca,
        [0x82, 0x87, 0xeb, 0x24, 0x8d, 0x4c, 0x50, 0xc0]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAsyncStreamCopier {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAsyncStreamCopier.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAsyncStreamCopierCoerce {
    /// Cheaply cast a value of this type from a `nsIAsyncStreamCopier`.
    fn coerce_from(v: &nsIAsyncStreamCopier) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAsyncStreamCopierCoerce for nsIAsyncStreamCopier {
    #[inline]
    fn coerce_from(v: &nsIAsyncStreamCopier) -> &Self {
        v
    }
}

impl nsIAsyncStreamCopier {
    /// Cast this `nsIAsyncStreamCopier` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAsyncStreamCopierCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAsyncStreamCopier {
    type Target = nsIRequest;
    #[inline]
    fn deref(&self) -> &nsIRequest {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIRequestCoerce> nsIAsyncStreamCopierCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAsyncStreamCopier) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAsyncStreamCopier
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAsyncStreamCopierVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIRequestVTable,

    /* void init (in nsIInputStream aSource, in nsIOutputStream aSink, in nsIEventTarget aTarget, in boolean aSourceBuffered, in boolean aSinkBuffered, in unsigned long aChunkSize, in boolean aCloseSource, in boolean aCloseSink); */
    pub Init: unsafe extern "system" fn (this: *const nsIAsyncStreamCopier, aSource: *const nsIInputStream, aSink: *const nsIOutputStream, aTarget: *const nsIEventTarget, aSourceBuffered: bool, aSinkBuffered: bool, aChunkSize: u32, aCloseSource: bool, aCloseSink: bool) -> ::nserror::nsresult,

    /* void asyncCopy (in nsIRequestObserver aObserver, in nsISupports aObserverContext); */
    pub AsyncCopy: unsafe extern "system" fn (this: *const nsIAsyncStreamCopier, aObserver: *const nsIRequestObserver, aObserverContext: *const nsISupports) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAsyncStreamCopier {

    /// ```text
    /// /**
    ///      * Initialize the stream copier.
    ///      *
    ///      * @param aSource
    ///      *        contains the data to be copied.
    ///      * @param aSink
    ///      *        specifies the destination for the data.
    ///      * @param aTarget
    ///      *        specifies the thread on which the copy will occur.  a null value
    ///      *        is permitted and will cause the copy to occur on an unspecified
    ///      *        background thread.
    ///      * @param aSourceBuffered
    ///      *        true if aSource implements ReadSegments.
    ///      * @param aSinkBuffered
    ///      *        true if aSink implements WriteSegments.
    ///      * @param aChunkSize
    ///      *        specifies how many bytes to read/write at a time.  this controls
    ///      *        the granularity of the copying.  it should match the segment size
    ///      *        of the "buffered" streams involved.
    ///      * @param aCloseSource
    ///      *        true if aSource should be closed after copying.
    ///      * @param aCloseSink
    ///      *        true if aSink should be closed after copying.
    ///      *
    ///      * NOTE: at least one of the streams must be buffered. If you do not know
    ///      * whether your streams are buffered, you should use nsIAsyncStreamCopier2
    ///      * instead.
    ///      */
    /// ```
    ///

    /// `void init (in nsIInputStream aSource, in nsIOutputStream aSink, in nsIEventTarget aTarget, in boolean aSourceBuffered, in boolean aSinkBuffered, in unsigned long aChunkSize, in boolean aCloseSource, in boolean aCloseSink);`
    #[inline]
    pub unsafe fn Init(&self, aSource: *const nsIInputStream, aSink: *const nsIOutputStream, aTarget: *const nsIEventTarget, aSourceBuffered: bool, aSinkBuffered: bool, aChunkSize: u32, aCloseSource: bool, aCloseSink: bool) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aSource, aSink, aTarget, aSourceBuffered, aSinkBuffered, aChunkSize, aCloseSource, aCloseSink)
    }


    /// ```text
    /// /**
    ///      * asyncCopy triggers the start of the copy.  The observer will be notified
    ///      * when the copy completes.
    ///      *
    ///      * @param aObserver
    ///      *        receives notifications.
    ///      * @param aObserverContext
    ///      *        passed to observer methods.
    ///      */
    /// ```
    ///

    /// `void asyncCopy (in nsIRequestObserver aObserver, in nsISupports aObserverContext);`
    #[inline]
    pub unsafe fn AsyncCopy(&self, aObserver: *const nsIRequestObserver, aObserverContext: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).AsyncCopy)(self, aObserver, aObserverContext)
    }


}


