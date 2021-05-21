//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/viewsource/nsIViewSourceChannel.idl
//


/// `interface nsIViewSourceChannel : nsIChannel`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIViewSourceChannel {
    vtable: *const nsIViewSourceChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIViewSourceChannel.
unsafe impl XpCom for nsIViewSourceChannel {
    const IID: nsIID = nsID(0x3e9800f8, 0xedb7, 0x4c9a,
        [0x92, 0x85, 0x09, 0xb4, 0xf0, 0x45, 0xb0, 0x19]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIViewSourceChannel {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIViewSourceChannel.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIViewSourceChannelCoerce {
    /// Cheaply cast a value of this type from a `nsIViewSourceChannel`.
    fn coerce_from(v: &nsIViewSourceChannel) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIViewSourceChannelCoerce for nsIViewSourceChannel {
    #[inline]
    fn coerce_from(v: &nsIViewSourceChannel) -> &Self {
        v
    }
}

impl nsIViewSourceChannel {
    /// Cast this `nsIViewSourceChannel` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIViewSourceChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIViewSourceChannel {
    type Target = nsIChannel;
    #[inline]
    fn deref(&self) -> &nsIChannel {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIChannelCoerce> nsIViewSourceChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIViewSourceChannel) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIViewSourceChannel
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIViewSourceChannelVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIChannelVTable,

    /* [must_use] attribute ACString originalContentType; */
    pub GetOriginalContentType: unsafe extern "system" fn (this: *const nsIViewSourceChannel, aOriginalContentType: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] attribute ACString originalContentType; */
    pub SetOriginalContentType: unsafe extern "system" fn (this: *const nsIViewSourceChannel, aOriginalContentType: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute boolean isSrcdocChannel; */
    pub GetIsSrcdocChannel: unsafe extern "system" fn (this: *const nsIViewSourceChannel, aIsSrcdocChannel: *mut bool) -> ::nserror::nsresult,

    /* [must_use] attribute nsIURI baseURI; */
    pub GetBaseURI: unsafe extern "system" fn (this: *const nsIViewSourceChannel, aBaseURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* [must_use] attribute nsIURI baseURI; */
    pub SetBaseURI: unsafe extern "system" fn (this: *const nsIViewSourceChannel, aBaseURI: *const nsIURI) -> ::nserror::nsresult,

    /* [nostdcall,notxpcom] nsIChannel getInnerChannel (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetInnerChannel: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIViewSourceChannel {

    /// ```text
    /// /**
    ///      * The actual (MIME) content type of the data.
    ///      *
    ///      * nsIViewSourceChannel returns a content type of
    ///      * "application/x-view-source" if you ask it for the contentType
    ///      * attribute.
    ///      *
    ///      * However, callers interested in finding out or setting the
    ///      * actual content type can utilize this attribute.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute ACString originalContentType;`
    #[inline]
    pub unsafe fn GetOriginalContentType(&self, aOriginalContentType: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetOriginalContentType)(self, aOriginalContentType)
    }


    /// ```text
    /// /**
    ///      * The actual (MIME) content type of the data.
    ///      *
    ///      * nsIViewSourceChannel returns a content type of
    ///      * "application/x-view-source" if you ask it for the contentType
    ///      * attribute.
    ///      *
    ///      * However, callers interested in finding out or setting the
    ///      * actual content type can utilize this attribute.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute ACString originalContentType;`
    #[inline]
    pub unsafe fn SetOriginalContentType(&self, aOriginalContentType: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetOriginalContentType)(self, aOriginalContentType)
    }


    /// ```text
    /// /**
    ///      * Whether the channel was created to view the source of a srcdoc document.
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute boolean isSrcdocChannel;`
    #[inline]
    pub unsafe fn GetIsSrcdocChannel(&self, aIsSrcdocChannel: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsSrcdocChannel)(self, aIsSrcdocChannel)
    }


    /// ```text
    /// /**
    ///      * Set to indicate the base URI.  If this channel is a srcdoc channel, it
    ///      * returns the base URI provided by the embedded channel.  It is used to
    ///      * provide an indication of the base URI in circumstances where it isn't
    ///      * otherwise recoverable.  Returns null when it isn't set and isn't a
    ///      * srcdoc channel.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute nsIURI baseURI;`
    #[inline]
    pub unsafe fn GetBaseURI(&self, aBaseURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetBaseURI)(self, aBaseURI)
    }


    /// ```text
    /// /**
    ///      * Set to indicate the base URI.  If this channel is a srcdoc channel, it
    ///      * returns the base URI provided by the embedded channel.  It is used to
    ///      * provide an indication of the base URI in circumstances where it isn't
    ///      * otherwise recoverable.  Returns null when it isn't set and isn't a
    ///      * srcdoc channel.
    ///      */
    /// ```
    ///

    /// `[must_use] attribute nsIURI baseURI;`
    #[inline]
    pub unsafe fn SetBaseURI(&self, aBaseURI: *const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).SetBaseURI)(self, aBaseURI)
    }


    /// ```text
    /// /**
    ///      * Get the inner channel wrapped by this nsIViewSourceChannel.
    ///      */
    /// ```
    ///

    /// `[nostdcall,notxpcom] nsIChannel getInnerChannel ();`
    const _GetInnerChannel: () = ();

}


