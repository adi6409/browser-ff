//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsISerializationHelper.idl
//


/// `interface nsISerializationHelper : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISerializationHelper {
    vtable: *const nsISerializationHelperVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISerializationHelper.
unsafe impl XpCom for nsISerializationHelper {
    const IID: nsIID = nsID(0x31654c0f, 0x35f3, 0x44c6,
        [0xb3, 0x1e, 0x37, 0xa1, 0x15, 0x16, 0xe6, 0xbc]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISerializationHelper {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISerializationHelper.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISerializationHelperCoerce {
    /// Cheaply cast a value of this type from a `nsISerializationHelper`.
    fn coerce_from(v: &nsISerializationHelper) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISerializationHelperCoerce for nsISerializationHelper {
    #[inline]
    fn coerce_from(v: &nsISerializationHelper) -> &Self {
        v
    }
}

impl nsISerializationHelper {
    /// Cast this `nsISerializationHelper` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISerializationHelperCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISerializationHelper {
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
impl<T: nsISupportsCoerce> nsISerializationHelperCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISerializationHelper) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISerializationHelper
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISerializationHelperVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* ACString serializeToString (in nsISerializable serializable); */
    pub SerializeToString: unsafe extern "system" fn (this: *const nsISerializationHelper, serializable: *const nsISerializable, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* nsISupports deserializeObject (in ACString input); */
    pub DeserializeObject: unsafe extern "system" fn (this: *const nsISerializationHelper, input: *const ::nsstring::nsACString, _retval: *mut *const nsISupports) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISerializationHelper {

    /// ```text
    /// /**
    ///    * Serialize the object to a base64 string. This string can be later passed
    ///    * as an input to deserializeObject method.
    ///    */
    /// ```
    ///

    /// `ACString serializeToString (in nsISerializable serializable);`
    #[inline]
    pub unsafe fn SerializeToString(&self, serializable: *const nsISerializable, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SerializeToString)(self, serializable, _retval)
    }


    /// ```text
    /// /**
    ///    * Takes base64 encoded string that cointains serialization of a single
    ///    * object. Most commonly, input is result of previous call to
    ///    * serializeToString.
    ///    */
    /// ```
    ///

    /// `nsISupports deserializeObject (in ACString input);`
    #[inline]
    pub unsafe fn DeserializeObject(&self, input: *const ::nsstring::nsACString, _retval: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).DeserializeObject)(self, input, _retval)
    }


}


