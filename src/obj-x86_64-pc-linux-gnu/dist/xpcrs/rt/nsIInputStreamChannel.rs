//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIInputStreamChannel.idl
//


/// `interface nsIInputStreamChannel : nsISupports`
///

/// ```text
/// /**
///  * nsIInputStreamChannel
///  *
///  * This interface provides methods to initialize an input stream channel.
///  * The input stream channel serves as a data pump for an input stream.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIInputStreamChannel {
    vtable: *const nsIInputStreamChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIInputStreamChannel.
unsafe impl XpCom for nsIInputStreamChannel {
    const IID: nsIID = nsID(0xea730238, 0x4bfd, 0x4015,
        [0x84, 0x89, 0x8f, 0x26, 0x4d, 0x05, 0xb3, 0x43]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIInputStreamChannel {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIInputStreamChannel.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIInputStreamChannelCoerce {
    /// Cheaply cast a value of this type from a `nsIInputStreamChannel`.
    fn coerce_from(v: &nsIInputStreamChannel) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIInputStreamChannelCoerce for nsIInputStreamChannel {
    #[inline]
    fn coerce_from(v: &nsIInputStreamChannel) -> &Self {
        v
    }
}

impl nsIInputStreamChannel {
    /// Cast this `nsIInputStreamChannel` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIInputStreamChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIInputStreamChannel {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsCoerce> nsIInputStreamChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInputStreamChannel) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIInputStreamChannel
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIInputStreamChannelVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void setURI (in nsIURI aURI); */
    pub SetURI: unsafe extern "system" fn (this: *const nsIInputStreamChannel, aURI: *const nsIURI) -> ::nserror::nsresult,

    /* attribute nsIInputStream contentStream; */
    pub GetContentStream: unsafe extern "system" fn (this: *const nsIInputStreamChannel, aContentStream: *mut*const nsIInputStream) -> ::nserror::nsresult,

    /* attribute nsIInputStream contentStream; */
    pub SetContentStream: unsafe extern "system" fn (this: *const nsIInputStreamChannel, aContentStream: *const nsIInputStream) -> ::nserror::nsresult,

    /* attribute AString srcdocData; */
    pub GetSrcdocData: unsafe extern "system" fn (this: *const nsIInputStreamChannel, aSrcdocData: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString srcdocData; */
    pub SetSrcdocData: unsafe extern "system" fn (this: *const nsIInputStreamChannel, aSrcdocData: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute boolean isSrcdocChannel; */
    pub GetIsSrcdocChannel: unsafe extern "system" fn (this: *const nsIInputStreamChannel, aIsSrcdocChannel: *mut bool) -> ::nserror::nsresult,

    /* attribute nsIURI baseURI; */
    pub GetBaseURI: unsafe extern "system" fn (this: *const nsIInputStreamChannel, aBaseURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* attribute nsIURI baseURI; */
    pub SetBaseURI: unsafe extern "system" fn (this: *const nsIInputStreamChannel, aBaseURI: *const nsIURI) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIInputStreamChannel {

    /// ```text
    /// /**
    ///      * Sets the URI for this channel.  This must be called before the
    ///      * channel is opened, and it may only be called once.
    ///      */
    /// ```
    ///

    /// `void setURI (in nsIURI aURI);`
    #[inline]
    pub unsafe fn SetURI(&self, aURI: *const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).SetURI)(self, aURI)
    }


    /// ```text
    /// /**
    ///      * Get/set the content stream
    ///      *
    ///      * This stream contains the data that will be pushed to the channel's
    ///      * stream listener.  If the stream is non-blocking and supports the
    ///      * nsIAsyncInputStream interface, then the stream will be read directly.
    ///      * Otherwise, the stream will be read on a background thread.
    ///      *
    ///      * This attribute must be set before the channel is opened, and it may
    ///      * only be set once.
    ///      *
    ///      * @throws NS_ERROR_IN_PROGRESS if the setter is called after the channel
    ///      * has been opened.
    ///      */
    /// ```
    ///

    /// `attribute nsIInputStream contentStream;`
    #[inline]
    pub unsafe fn GetContentStream(&self, aContentStream: *mut*const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).GetContentStream)(self, aContentStream)
    }


    /// ```text
    /// /**
    ///      * Get/set the content stream
    ///      *
    ///      * This stream contains the data that will be pushed to the channel's
    ///      * stream listener.  If the stream is non-blocking and supports the
    ///      * nsIAsyncInputStream interface, then the stream will be read directly.
    ///      * Otherwise, the stream will be read on a background thread.
    ///      *
    ///      * This attribute must be set before the channel is opened, and it may
    ///      * only be set once.
    ///      *
    ///      * @throws NS_ERROR_IN_PROGRESS if the setter is called after the channel
    ///      * has been opened.
    ///      */
    /// ```
    ///

    /// `attribute nsIInputStream contentStream;`
    #[inline]
    pub unsafe fn SetContentStream(&self, aContentStream: *const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).SetContentStream)(self, aContentStream)
    }


    /// ```text
    /// /**
    ///      * Get/set the srcdoc data string.  When the input stream channel is
    ///      * created to load a srcdoc iframe, this is set to hold the value of the
    ///      * srcdoc attribute.
    ///      *
    ///      * This should be the same value used to create contentStream, but this is
    ///      * not checked.
    ///      *
    ///      * Changing the value of this attribute will not otherwise affect the
    ///      * functionality of the channel or input stream.
    ///      */
    /// ```
    ///

    /// `attribute AString srcdocData;`
    #[inline]
    pub unsafe fn GetSrcdocData(&self, aSrcdocData: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSrcdocData)(self, aSrcdocData)
    }


    /// ```text
    /// /**
    ///      * Get/set the srcdoc data string.  When the input stream channel is
    ///      * created to load a srcdoc iframe, this is set to hold the value of the
    ///      * srcdoc attribute.
    ///      *
    ///      * This should be the same value used to create contentStream, but this is
    ///      * not checked.
    ///      *
    ///      * Changing the value of this attribute will not otherwise affect the
    ///      * functionality of the channel or input stream.
    ///      */
    /// ```
    ///

    /// `attribute AString srcdocData;`
    #[inline]
    pub unsafe fn SetSrcdocData(&self, aSrcdocData: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetSrcdocData)(self, aSrcdocData)
    }


    /// ```text
    /// /**
    ///      * Returns true if srcdocData has been set within the channel.
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean isSrcdocChannel;`
    #[inline]
    pub unsafe fn GetIsSrcdocChannel(&self, aIsSrcdocChannel: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsSrcdocChannel)(self, aIsSrcdocChannel)
    }


    /// ```text
    /// /**
    ///      * The base URI to be used for the channel.  Used when the base URI cannot
    ///      * be inferred by other means, for example when this is a srcdoc channel.
    ///      */
    /// ```
    ///

    /// `attribute nsIURI baseURI;`
    #[inline]
    pub unsafe fn GetBaseURI(&self, aBaseURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetBaseURI)(self, aBaseURI)
    }


    /// ```text
    /// /**
    ///      * The base URI to be used for the channel.  Used when the base URI cannot
    ///      * be inferred by other means, for example when this is a srcdoc channel.
    ///      */
    /// ```
    ///

    /// `attribute nsIURI baseURI;`
    #[inline]
    pub unsafe fn SetBaseURI(&self, aBaseURI: *const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).SetBaseURI)(self, aBaseURI)
    }


}


