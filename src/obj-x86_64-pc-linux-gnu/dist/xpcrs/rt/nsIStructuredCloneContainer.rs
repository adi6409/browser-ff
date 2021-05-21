//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIStructuredCloneContainer.idl
//


/// `interface nsIStructuredCloneContainer : nsISupports`
///

/// ```text
/// /**
///  * This interface acts as a container for an object serialized using the
///  * structured clone algorithm.
///  *
///  * You can copy an object into an nsIStructuredCloneContainer using
///  * initFromJSVal or initFromBase64.  It's an error to initialize an
///  * nsIStructuredCloneContainer more than once.
///  *
///  * Once you've initialized the container, you can get a copy of the object it
///  * stores by calling deserializeToVariant.  You can also get a base-64-encoded
///  * string containing a copy of the container's serialized data, using
///  * getDataAsBase64.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIStructuredCloneContainer {
    vtable: *const nsIStructuredCloneContainerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIStructuredCloneContainer.
unsafe impl XpCom for nsIStructuredCloneContainer {
    const IID: nsIID = nsID(0xc664aae7, 0x0d67, 0x4155,
        [0xa2, 0xdd, 0xa3, 0x86, 0x17, 0x78, 0x62, 0x6f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIStructuredCloneContainer {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIStructuredCloneContainer.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIStructuredCloneContainerCoerce {
    /// Cheaply cast a value of this type from a `nsIStructuredCloneContainer`.
    fn coerce_from(v: &nsIStructuredCloneContainer) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIStructuredCloneContainerCoerce for nsIStructuredCloneContainer {
    #[inline]
    fn coerce_from(v: &nsIStructuredCloneContainer) -> &Self {
        v
    }
}

impl nsIStructuredCloneContainer {
    /// Cast this `nsIStructuredCloneContainer` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIStructuredCloneContainerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIStructuredCloneContainer {
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
impl<T: nsISupportsCoerce> nsIStructuredCloneContainerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStructuredCloneContainer) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIStructuredCloneContainer
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIStructuredCloneContainerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext,noscript] void initFromJSVal (in jsval aData); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub InitFromJSVal: *const ::libc::c_void,

    /* void initFromBase64 (in AString aData, in unsigned long aFormatVersion); */
    pub InitFromBase64: unsafe extern "system" fn (this: *const nsIStructuredCloneContainer, aData: *const ::nsstring::nsAString, aFormatVersion: u32) -> ::nserror::nsresult,

    /* [implicit_jscontext] jsval deserializeToJsval (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub DeserializeToJsval: *const ::libc::c_void,

    /* [implicit_jscontext] nsIVariant deserializeToVariant (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub DeserializeToVariant: *const ::libc::c_void,

    /* AString getDataAsBase64 (); */
    pub GetDataAsBase64: unsafe extern "system" fn (this: *const nsIStructuredCloneContainer, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute unsigned long long serializedNBytes; */
    pub GetSerializedNBytes: unsafe extern "system" fn (this: *const nsIStructuredCloneContainer, aSerializedNBytes: *mut u64) -> ::nserror::nsresult,

    /* readonly attribute unsigned long formatVersion; */
    pub GetFormatVersion: unsafe extern "system" fn (this: *const nsIStructuredCloneContainer, aFormatVersion: *mut u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIStructuredCloneContainer {

    /// ```text
    /// /**
    ///    * Initialize this structured clone container so it contains a clone of the
    ///    * given jsval.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext,noscript] void initFromJSVal (in jsval aData);`
    const _InitFromJSVal: () = ();

    /// ```text
    /// /**
    ///    * Initialize this structured clone container from a base-64-encoded byte
    ///    * stream, stored in aData.  aFormatVersion should be the version of the
    ///    * structured clone algorithm which was used to generate aData.
    ///    */
    /// ```
    ///

    /// `void initFromBase64 (in AString aData, in unsigned long aFormatVersion);`
    #[inline]
    pub unsafe fn InitFromBase64(&self, aData: *const ::nsstring::nsAString, aFormatVersion: u32) -> ::nserror::nsresult {
        ((*self.vtable).InitFromBase64)(self, aData, aFormatVersion)
    }


    /// ```text
    /// /**
    ///    * Deserializes this structured clone container returning it as a jsval.
    ///    * Can be called on main and worker threads.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] jsval deserializeToJsval ();`
    const _DeserializeToJsval: () = ();

    /// ```text
    /// /**
    ///    * Deserialize the object this container holds, returning it wrapped as
    ///    * an nsIVariant.
    ///    * Main thread only!
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] nsIVariant deserializeToVariant ();`
    const _DeserializeToVariant: () = ();

    /// ```text
    /// /**
    ///    * Get this structured clone container's data as a base-64-encoded string.
    ///    */
    /// ```
    ///

    /// `AString getDataAsBase64 ();`
    #[inline]
    pub unsafe fn GetDataAsBase64(&self, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetDataAsBase64)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Get the size in bytes of this container's serialized data.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long long serializedNBytes;`
    #[inline]
    pub unsafe fn GetSerializedNBytes(&self, aSerializedNBytes: *mut u64) -> ::nserror::nsresult {
        ((*self.vtable).GetSerializedNBytes)(self, aSerializedNBytes)
    }


    /// ```text
    /// /**
    ///    * Get the version of the structured clone algorithm which was used to
    ///    * generate this container's serialized buffer.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long formatVersion;`
    #[inline]
    pub unsafe fn GetFormatVersion(&self, aFormatVersion: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetFormatVersion)(self, aFormatVersion)
    }


}


