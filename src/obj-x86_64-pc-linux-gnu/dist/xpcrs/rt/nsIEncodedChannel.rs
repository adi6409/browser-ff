//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIEncodedChannel.idl
//


/// `interface nsIEncodedChannel : nsISupports`
///

/// ```text
/// /**
///  * A channel interface which allows special handling of encoded content
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIEncodedChannel {
    vtable: *const nsIEncodedChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIEncodedChannel.
unsafe impl XpCom for nsIEncodedChannel {
    const IID: nsIID = nsID(0x29c29ce6, 0x8ce4, 0x45e6,
        [0x8d, 0x60, 0x36, 0xc8, 0xfa, 0x3e, 0x25, 0x5b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIEncodedChannel {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIEncodedChannel.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIEncodedChannelCoerce {
    /// Cheaply cast a value of this type from a `nsIEncodedChannel`.
    fn coerce_from(v: &nsIEncodedChannel) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIEncodedChannelCoerce for nsIEncodedChannel {
    #[inline]
    fn coerce_from(v: &nsIEncodedChannel) -> &Self {
        v
    }
}

impl nsIEncodedChannel {
    /// Cast this `nsIEncodedChannel` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIEncodedChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIEncodedChannel {
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
impl<T: nsISupportsCoerce> nsIEncodedChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEncodedChannel) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIEncodedChannel
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIEncodedChannelVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIUTF8StringEnumerator contentEncodings; */
    pub GetContentEncodings: unsafe extern "system" fn (this: *const nsIEncodedChannel, aContentEncodings: *mut*const nsIUTF8StringEnumerator) -> ::nserror::nsresult,

    /* attribute boolean applyConversion; */
    pub GetApplyConversion: unsafe extern "system" fn (this: *const nsIEncodedChannel, aApplyConversion: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean applyConversion; */
    pub SetApplyConversion: unsafe extern "system" fn (this: *const nsIEncodedChannel, aApplyConversion: bool) -> ::nserror::nsresult,

    /* void doApplyContentConversions (in nsIStreamListener aNextListener, out nsIStreamListener aNewNextListener, in nsISupports aCtxt); */
    pub DoApplyContentConversions: unsafe extern "system" fn (this: *const nsIEncodedChannel, aNextListener: *const nsIStreamListener, aNewNextListener: *mut*const nsIStreamListener, aCtxt: *const nsISupports) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIEncodedChannel {

    /// ```text
    /// /**
    ///      * This attribute holds the MIME types corresponding to the content
    ///      * encodings on the channel.  The enumerator returns nsISupportsCString
    ///      * objects.  The first one corresponds to the outermost encoding on the
    ///      * channel and then we work our way inward.  "identity" is skipped and not
    ///      * represented on the list.  Unknown encodings make the enumeration stop.
    ///      * If you want the actual Content-Encoding value, use
    ///      * getResponseHeader("Content-Encoding").
    ///      *
    ///      * When there is no Content-Encoding header, this property is null.
    ///      *
    ///      * Modifying the Content-Encoding header on the channel will cause
    ///      * this enumerator to have undefined behavior.  Don't do it.
    ///      *
    ///      * Also note that contentEncodings only exist during or after OnStartRequest.
    ///      * Calling contentEncodings before OnStartRequest is an error.
    ///      */
    /// ```
    ///

    /// `readonly attribute nsIUTF8StringEnumerator contentEncodings;`
    #[inline]
    pub unsafe fn GetContentEncodings(&self, aContentEncodings: *mut*const nsIUTF8StringEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).GetContentEncodings)(self, aContentEncodings)
    }


    /// ```text
    /// /**
    ///      * This attribute controls whether or not content conversion should be
    ///      * done per the Content-Encoding response header.  applyConversion can only
    ///      * be set before or during OnStartRequest.  Calling this during
    ///      * OnDataAvailable is an error.
    ///      *
    ///      * TRUE by default.
    ///      */
    /// ```
    ///

    /// `attribute boolean applyConversion;`
    #[inline]
    pub unsafe fn GetApplyConversion(&self, aApplyConversion: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetApplyConversion)(self, aApplyConversion)
    }


    /// ```text
    /// /**
    ///      * This attribute controls whether or not content conversion should be
    ///      * done per the Content-Encoding response header.  applyConversion can only
    ///      * be set before or during OnStartRequest.  Calling this during
    ///      * OnDataAvailable is an error.
    ///      *
    ///      * TRUE by default.
    ///      */
    /// ```
    ///

    /// `attribute boolean applyConversion;`
    #[inline]
    pub unsafe fn SetApplyConversion(&self, aApplyConversion: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetApplyConversion)(self, aApplyConversion)
    }


    /// ```text
    /// /**
    ///      * This function will start converters if they are available.
    ///      * aNewNextListener will be nullptr if no converter is available.
    ///      */
    /// ```
    ///

    /// `void doApplyContentConversions (in nsIStreamListener aNextListener, out nsIStreamListener aNewNextListener, in nsISupports aCtxt);`
    #[inline]
    pub unsafe fn DoApplyContentConversions(&self, aNextListener: *const nsIStreamListener, aNewNextListener: *mut*const nsIStreamListener, aCtxt: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).DoApplyContentConversions)(self, aNextListener, aNewNextListener, aCtxt)
    }


}


