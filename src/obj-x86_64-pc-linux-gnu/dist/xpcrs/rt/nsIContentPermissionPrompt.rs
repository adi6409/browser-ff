//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIContentPermissionPrompt.idl
//


/// `interface nsIContentPermissionType : nsISupports`
///

/// ```text
/// /**
///  *  Interface provides the request type and its access.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIContentPermissionType {
    vtable: *const nsIContentPermissionTypeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIContentPermissionType.
unsafe impl XpCom for nsIContentPermissionType {
    const IID: nsIID = nsID(0xef4db3b8, 0xca9c, 0x4b1d,
        [0x8f, 0x81, 0xfd, 0x88, 0xec, 0x32, 0xaf, 0x13]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIContentPermissionType {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIContentPermissionType.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIContentPermissionTypeCoerce {
    /// Cheaply cast a value of this type from a `nsIContentPermissionType`.
    fn coerce_from(v: &nsIContentPermissionType) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIContentPermissionTypeCoerce for nsIContentPermissionType {
    #[inline]
    fn coerce_from(v: &nsIContentPermissionType) -> &Self {
        v
    }
}

impl nsIContentPermissionType {
    /// Cast this `nsIContentPermissionType` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIContentPermissionTypeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIContentPermissionType {
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
impl<T: nsISupportsCoerce> nsIContentPermissionTypeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentPermissionType) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIContentPermissionType
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIContentPermissionTypeVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString type; */
    pub GetType: unsafe extern "system" fn (this: *const nsIContentPermissionType, aType: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute nsIArray options; */
    pub GetOptions: unsafe extern "system" fn (this: *const nsIContentPermissionType, aOptions: *mut*const nsIArray) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIContentPermissionType {

    /// ```text
    /// /**
    ///    *  The type of the permission request, such as
    ///    *  "geolocation".
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString type;`
    #[inline]
    pub unsafe fn GetType(&self, aType: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetType)(self, aType)
    }


    /// ```text
    /// /**
    ///    * The array of available options.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIArray options;`
    #[inline]
    pub unsafe fn GetOptions(&self, aOptions: *mut*const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetOptions)(self, aOptions)
    }


}


/// `interface nsIContentPermissionRequest : nsISupports`
///

/// ```text
/// /**
///  * Interface allows access to a content to request
///  * permission to perform a privileged operation such as
///  * geolocation.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIContentPermissionRequest {
    vtable: *const nsIContentPermissionRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIContentPermissionRequest.
unsafe impl XpCom for nsIContentPermissionRequest {
    const IID: nsIID = nsID(0x875733da, 0x0ac0, 0x4a26,
        [0x8c, 0x76, 0x70, 0xa3, 0x08, 0x76, 0xbe, 0x46]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIContentPermissionRequest {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIContentPermissionRequest.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIContentPermissionRequestCoerce {
    /// Cheaply cast a value of this type from a `nsIContentPermissionRequest`.
    fn coerce_from(v: &nsIContentPermissionRequest) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIContentPermissionRequestCoerce for nsIContentPermissionRequest {
    #[inline]
    fn coerce_from(v: &nsIContentPermissionRequest) -> &Self {
        v
    }
}

impl nsIContentPermissionRequest {
    /// Cast this `nsIContentPermissionRequest` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIContentPermissionRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIContentPermissionRequest {
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
impl<T: nsISupportsCoerce> nsIContentPermissionRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentPermissionRequest) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIContentPermissionRequest
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIContentPermissionRequestVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIArray types; */
    pub GetTypes: unsafe extern "system" fn (this: *const nsIContentPermissionRequest, aTypes: *mut*const nsIArray) -> ::nserror::nsresult,

    /* readonly attribute nsIPrincipal principal; */
    pub GetPrincipal: unsafe extern "system" fn (this: *const nsIContentPermissionRequest, aPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult,

    /* readonly attribute nsIPrincipal topLevelPrincipal; */
    pub GetTopLevelPrincipal: unsafe extern "system" fn (this: *const nsIContentPermissionRequest, aTopLevelPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult,

    /* readonly attribute mozIDOMWindow window; */
    pub GetWindow: unsafe extern "system" fn (this: *const nsIContentPermissionRequest, aWindow: *mut*const mozIDOMWindow) -> ::nserror::nsresult,

    /* readonly attribute Element element; */
    pub GetElement: unsafe extern "system" fn (this: *const nsIContentPermissionRequest, aElement: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* readonly attribute boolean isHandlingUserInput; */
    pub GetIsHandlingUserInput: unsafe extern "system" fn (this: *const nsIContentPermissionRequest, aIsHandlingUserInput: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean maybeUnsafePermissionDelegate; */
    pub GetMaybeUnsafePermissionDelegate: unsafe extern "system" fn (this: *const nsIContentPermissionRequest, aMaybeUnsafePermissionDelegate: *mut bool) -> ::nserror::nsresult,

    /* nsIPrincipal getDelegatePrincipal (in ACString aType); */
    pub GetDelegatePrincipal: unsafe extern "system" fn (this: *const nsIContentPermissionRequest, aType: *const ::nsstring::nsACString, _retval: *mut*const nsIPrincipal) -> ::nserror::nsresult,

    /* [can_run_script] void cancel (); */
    pub Cancel: unsafe extern "system" fn (this: *const nsIContentPermissionRequest) -> ::nserror::nsresult,

    /* [can_run_script] void allow ([optional] in jsval choices); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub Allow: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIContentPermissionRequest {

    /// ```text
    /// /**
    ///    *  The array will include the request types. Elements of this array are
    ///    *  nsIContentPermissionType object.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIArray types;`
    #[inline]
    pub unsafe fn GetTypes(&self, aTypes: *mut*const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetTypes)(self, aTypes)
    }



    /// `readonly attribute nsIPrincipal principal;`
    #[inline]
    pub unsafe fn GetPrincipal(&self, aPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).GetPrincipal)(self, aPrincipal)
    }



    /// `readonly attribute nsIPrincipal topLevelPrincipal;`
    #[inline]
    pub unsafe fn GetTopLevelPrincipal(&self, aTopLevelPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).GetTopLevelPrincipal)(self, aTopLevelPrincipal)
    }


    /// ```text
    /// /**
    ///    *  The window or element that the permission request was
    ///    *  originated in.  Typically the element will be non-null
    ///    *  in when using out of process content.  window or
    ///    *  element can be null but not both.
    ///    */
    /// ```
    ///

    /// `readonly attribute mozIDOMWindow window;`
    #[inline]
    pub unsafe fn GetWindow(&self, aWindow: *mut*const mozIDOMWindow) -> ::nserror::nsresult {
        ((*self.vtable).GetWindow)(self, aWindow)
    }



    /// `readonly attribute Element element;`
    #[inline]
    pub unsafe fn GetElement(&self, aElement: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetElement)(self, aElement)
    }



    /// `readonly attribute boolean isHandlingUserInput;`
    #[inline]
    pub unsafe fn GetIsHandlingUserInput(&self, aIsHandlingUserInput: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsHandlingUserInput)(self, aIsHandlingUserInput)
    }



    /// `readonly attribute boolean maybeUnsafePermissionDelegate;`
    #[inline]
    pub unsafe fn GetMaybeUnsafePermissionDelegate(&self, aMaybeUnsafePermissionDelegate: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetMaybeUnsafePermissionDelegate)(self, aMaybeUnsafePermissionDelegate)
    }



    /// `nsIPrincipal getDelegatePrincipal (in ACString aType);`
    #[inline]
    pub unsafe fn GetDelegatePrincipal(&self, aType: *const ::nsstring::nsACString, _retval: *mut*const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).GetDelegatePrincipal)(self, aType, _retval)
    }


    /// ```text
    /// /**
    ///    * allow or cancel the request
    ///    */
    /// ```
    ///

    /// `[can_run_script] void cancel ();`
    #[inline]
    pub unsafe fn Cancel(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Cancel)(self, )
    }



    /// `[can_run_script] void allow ([optional] in jsval choices);`
    const _Allow: () = ();

}


/// `interface nsIContentPermissionPrompt : nsISupports`
///

/// ```text
/// /**
///  * Interface provides a way for the application to handle
///  * the UI prompts associated with geo position.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIContentPermissionPrompt {
    vtable: *const nsIContentPermissionPromptVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIContentPermissionPrompt.
unsafe impl XpCom for nsIContentPermissionPrompt {
    const IID: nsIID = nsID(0xf72de90d, 0xe954, 0x4e69,
        [0x9a, 0x61, 0x91, 0x73, 0x03, 0x02, 0x93, 0x01]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIContentPermissionPrompt {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIContentPermissionPrompt.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIContentPermissionPromptCoerce {
    /// Cheaply cast a value of this type from a `nsIContentPermissionPrompt`.
    fn coerce_from(v: &nsIContentPermissionPrompt) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIContentPermissionPromptCoerce for nsIContentPermissionPrompt {
    #[inline]
    fn coerce_from(v: &nsIContentPermissionPrompt) -> &Self {
        v
    }
}

impl nsIContentPermissionPrompt {
    /// Cast this `nsIContentPermissionPrompt` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIContentPermissionPromptCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIContentPermissionPrompt {
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
impl<T: nsISupportsCoerce> nsIContentPermissionPromptCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentPermissionPrompt) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIContentPermissionPrompt
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIContentPermissionPromptVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void prompt (in nsIContentPermissionRequest request); */
    pub Prompt: unsafe extern "system" fn (this: *const nsIContentPermissionPrompt, request: *const nsIContentPermissionRequest) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIContentPermissionPrompt {

    /// ```text
    /// /**
    ///    * Called when a request has been made to access
    ///    * privileged content apis
    ///    */
    /// ```
    ///

    /// `void prompt (in nsIContentPermissionRequest request);`
    #[inline]
    pub unsafe fn Prompt(&self, request: *const nsIContentPermissionRequest) -> ::nserror::nsresult {
        ((*self.vtable).Prompt)(self, request)
    }


}


