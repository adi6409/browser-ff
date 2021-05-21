//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibilityService.idl
//


/// `interface nsIAccessibilityService : nsISupports`
///

/// ```text
/// /**
///  * An interface for in-process accessibility clients wishing to get an
///  * nsIAccessible for a given DOM node.  More documentation at:
///  *   http://www.mozilla.org/projects/ui/accessibility
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessibilityService {
    vtable: *const nsIAccessibilityServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessibilityService.
unsafe impl XpCom for nsIAccessibilityService {
    const IID: nsIID = nsID(0x2188e3a0, 0xc88e, 0x11e7,
        [0x8f, 0x1a, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessibilityService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessibilityService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessibilityServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessibilityService`.
    fn coerce_from(v: &nsIAccessibilityService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessibilityServiceCoerce for nsIAccessibilityService {
    #[inline]
    fn coerce_from(v: &nsIAccessibilityService) -> &Self {
        v
    }
}

impl nsIAccessibilityService {
    /// Cast this `nsIAccessibilityService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessibilityServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessibilityService {
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
impl<T: nsISupportsCoerce> nsIAccessibilityServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibilityService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessibilityService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessibilityServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIAccessible getApplicationAccessible (); */
    pub GetApplicationAccessible: unsafe extern "system" fn (this: *const nsIAccessibilityService, _retval: *mut*const nsIAccessible) -> ::nserror::nsresult,

    /* nsIAccessible getAccessibleFor (in Node aNode); */
    pub GetAccessibleFor: unsafe extern "system" fn (this: *const nsIAccessibilityService, aNode: *const libc::c_void, _retval: *mut*const nsIAccessible) -> ::nserror::nsresult,

    /* AString getStringRole (in unsigned long aRole); */
    pub GetStringRole: unsafe extern "system" fn (this: *const nsIAccessibilityService, aRole: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* nsISupports getStringStates (in unsigned long aStates, in unsigned long aExtraStates); */
    pub GetStringStates: unsafe extern "system" fn (this: *const nsIAccessibilityService, aStates: u32, aExtraStates: u32, _retval: *mut *const nsISupports) -> ::nserror::nsresult,

    /* AString getStringEventType (in unsigned long aEventType); */
    pub GetStringEventType: unsafe extern "system" fn (this: *const nsIAccessibilityService, aEventType: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString getStringRelationType (in unsigned long aRelationType); */
    pub GetStringRelationType: unsafe extern "system" fn (this: *const nsIAccessibilityService, aRelationType: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* nsIAccessible getAccessibleFromCache (in Node aNode); */
    pub GetAccessibleFromCache: unsafe extern "system" fn (this: *const nsIAccessibilityService, aNode: *const libc::c_void, _retval: *mut*const nsIAccessible) -> ::nserror::nsresult,

    /* nsIAccessiblePivot createAccessiblePivot (in nsIAccessible aRoot); */
    pub CreateAccessiblePivot: unsafe extern "system" fn (this: *const nsIAccessibilityService, aRoot: *const nsIAccessible, _retval: *mut*const nsIAccessiblePivot) -> ::nserror::nsresult,

    /* void setLogging (in ACString aModules); */
    pub SetLogging: unsafe extern "system" fn (this: *const nsIAccessibilityService, aModules: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* boolean isLogged (in AString aModule); */
    pub IsLogged: unsafe extern "system" fn (this: *const nsIAccessibilityService, aModule: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult,

    /* AString getConsumers (); */
    pub GetConsumers: unsafe extern "system" fn (this: *const nsIAccessibilityService, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessibilityService {

    /// ```text
    /// /**
    ///    * Return application accessible.
    ///    */
    /// ```
    ///

    /// `nsIAccessible getApplicationAccessible ();`
    #[inline]
    pub unsafe fn GetApplicationAccessible(&self, _retval: *mut*const nsIAccessible) -> ::nserror::nsresult {
        ((*self.vtable).GetApplicationAccessible)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Return an nsIAccessible for a DOM node in pres shell 0.
    ///    * Create a new accessible of the appropriate type if necessary,
    ///    * or use one from the accessibility cache if it already exists.
    ///    * @param aNode The DOM node to get an accessible for.
    ///    * @return The nsIAccessible for the given DOM node.
    ///    */
    /// ```
    ///

    /// `nsIAccessible getAccessibleFor (in Node aNode);`
    #[inline]
    pub unsafe fn GetAccessibleFor(&self, aNode: *const libc::c_void, _retval: *mut*const nsIAccessible) -> ::nserror::nsresult {
        ((*self.vtable).GetAccessibleFor)(self, aNode, _retval)
    }


    /// ```text
    /// /**
    ///     * Returns accessible role as a string.
    ///     *
    ///     * @param aRole - the accessible role constants.
    ///     */
    /// ```
    ///

    /// `AString getStringRole (in unsigned long aRole);`
    #[inline]
    pub unsafe fn GetStringRole(&self, aRole: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetStringRole)(self, aRole, _retval)
    }


    /// ```text
    /// /**
    ///     * Returns list which contains accessible states as a strings.
    ///     *
    ///     * @param aStates - accessible states.
    ///     * @param aExtraStates - accessible extra states.
    ///     */
    /// ```
    ///

    /// `nsISupports getStringStates (in unsigned long aStates, in unsigned long aExtraStates);`
    #[inline]
    pub unsafe fn GetStringStates(&self, aStates: u32, aExtraStates: u32, _retval: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetStringStates)(self, aStates, aExtraStates, _retval)
    }


    /// ```text
    /// /**
    ///    * Get the type of accessible event as a string.
    ///    *
    ///    * @param aEventType - the accessible event type constant
    ///    * @return - accessible event type presented as human readable string
    ///    */
    /// ```
    ///

    /// `AString getStringEventType (in unsigned long aEventType);`
    #[inline]
    pub unsafe fn GetStringEventType(&self, aEventType: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetStringEventType)(self, aEventType, _retval)
    }


    /// ```text
    /// /**
    ///    * Get the type of accessible relation as a string.
    ///    *
    ///    * @param aRelationType - the accessible relation type constant
    ///    * @return - accessible relation type presented as human readable string
    ///    */
    /// ```
    ///

    /// `AString getStringRelationType (in unsigned long aRelationType);`
    #[inline]
    pub unsafe fn GetStringRelationType(&self, aRelationType: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetStringRelationType)(self, aRelationType, _retval)
    }


    /// ```text
    /// /**
    ///    * Return an accessible for the given DOM node from the cache.
    ///    * @note  the method is intended for testing purposes
    ///    *
    ///    * @param aNode  [in] the DOM node to get an accessible for
    ///    *
    ///    * @return       cached accessible for the given DOM node if any
    ///    */
    /// ```
    ///

    /// `nsIAccessible getAccessibleFromCache (in Node aNode);`
    #[inline]
    pub unsafe fn GetAccessibleFromCache(&self, aNode: *const libc::c_void, _retval: *mut*const nsIAccessible) -> ::nserror::nsresult {
        ((*self.vtable).GetAccessibleFromCache)(self, aNode, _retval)
    }


    /// ```text
    /// /**
    ///    * Create a new pivot for tracking a position and traversing a subtree.
    ///    *
    ///    * @param aRoot [in] the accessible root for the pivot
    ///    * @return a new pivot
    ///    */
    /// ```
    ///

    /// `nsIAccessiblePivot createAccessiblePivot (in nsIAccessible aRoot);`
    #[inline]
    pub unsafe fn CreateAccessiblePivot(&self, aRoot: *const nsIAccessible, _retval: *mut*const nsIAccessiblePivot) -> ::nserror::nsresult {
        ((*self.vtable).CreateAccessiblePivot)(self, aRoot, _retval)
    }


    /// ```text
    /// /**
    ///    * Enable logging for the given modules, all other modules aren't logged.
    ///    *
    ///    * @param aModules [in] list of modules, format is comma separated list
    ///    *                      like 'docload,doccreate'.
    ///    * @note Works on debug build only.
    ///    * @see Logging.cpp for list of possible values.
    ///    */
    /// ```
    ///

    /// `void setLogging (in ACString aModules);`
    #[inline]
    pub unsafe fn SetLogging(&self, aModules: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetLogging)(self, aModules)
    }


    /// ```text
    /// /**
    ///    * Return true if the given module is logged.
    ///    */
    /// ```
    ///

    /// `boolean isLogged (in AString aModule);`
    #[inline]
    pub unsafe fn IsLogged(&self, aModule: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsLogged)(self, aModule, _retval)
    }


    /// ```text
    /// /**
    ///    * Get the current accessibility service consumers.
    ///    * @returns a JSON string representing the accessibility service consumers.
    ///    */
    /// ```
    ///

    /// `AString getConsumers ();`
    #[inline]
    pub unsafe fn GetConsumers(&self, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetConsumers)(self, _retval)
    }


}


