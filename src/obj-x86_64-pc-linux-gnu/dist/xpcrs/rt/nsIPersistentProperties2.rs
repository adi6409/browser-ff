//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIPersistentProperties2.idl
//


/// `interface nsIPropertyElement : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPropertyElement {
    vtable: *const nsIPropertyElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPropertyElement.
unsafe impl XpCom for nsIPropertyElement {
    const IID: nsIID = nsID(0x283ee646, 0x1aef, 0x11d4,
        [0x98, 0xb3, 0x00, 0xc0, 0x4f, 0xa0, 0xce, 0x9a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPropertyElement {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPropertyElement.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPropertyElementCoerce {
    /// Cheaply cast a value of this type from a `nsIPropertyElement`.
    fn coerce_from(v: &nsIPropertyElement) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPropertyElementCoerce for nsIPropertyElement {
    #[inline]
    fn coerce_from(v: &nsIPropertyElement) -> &Self {
        v
    }
}

impl nsIPropertyElement {
    /// Cast this `nsIPropertyElement` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPropertyElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPropertyElement {
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
impl<T: nsISupportsCoerce> nsIPropertyElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPropertyElement) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPropertyElement
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPropertyElementVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute AUTF8String key; */
    pub GetKey: unsafe extern "system" fn (this: *const nsIPropertyElement, aKey: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AUTF8String key; */
    pub SetKey: unsafe extern "system" fn (this: *const nsIPropertyElement, aKey: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AString value; */
    pub GetValue: unsafe extern "system" fn (this: *const nsIPropertyElement, aValue: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString value; */
    pub SetValue: unsafe extern "system" fn (this: *const nsIPropertyElement, aValue: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPropertyElement {


    /// `attribute AUTF8String key;`
    #[inline]
    pub unsafe fn GetKey(&self, aKey: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetKey)(self, aKey)
    }



    /// `attribute AUTF8String key;`
    #[inline]
    pub unsafe fn SetKey(&self, aKey: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetKey)(self, aKey)
    }



    /// `attribute AString value;`
    #[inline]
    pub unsafe fn GetValue(&self, aValue: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetValue)(self, aValue)
    }



    /// `attribute AString value;`
    #[inline]
    pub unsafe fn SetValue(&self, aValue: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetValue)(self, aValue)
    }


}


/// `interface nsIPersistentProperties : nsIProperties`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPersistentProperties {
    vtable: *const nsIPersistentPropertiesVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPersistentProperties.
unsafe impl XpCom for nsIPersistentProperties {
    const IID: nsIID = nsID(0x706867af, 0x0400, 0x4faa,
        [0xbe, 0xb1, 0x0d, 0xae, 0x87, 0x30, 0x87, 0x84]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPersistentProperties {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPersistentProperties.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPersistentPropertiesCoerce {
    /// Cheaply cast a value of this type from a `nsIPersistentProperties`.
    fn coerce_from(v: &nsIPersistentProperties) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPersistentPropertiesCoerce for nsIPersistentProperties {
    #[inline]
    fn coerce_from(v: &nsIPersistentProperties) -> &Self {
        v
    }
}

impl nsIPersistentProperties {
    /// Cast this `nsIPersistentProperties` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPersistentPropertiesCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPersistentProperties {
    type Target = nsIProperties;
    #[inline]
    fn deref(&self) -> &nsIProperties {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIPropertiesCoerce> nsIPersistentPropertiesCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPersistentProperties) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPersistentProperties
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPersistentPropertiesVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIPropertiesVTable,

    /* void load (in nsIInputStream input); */
    pub Load: unsafe extern "system" fn (this: *const nsIPersistentProperties, input: *const nsIInputStream) -> ::nserror::nsresult,

    /* void save (in nsIOutputStream output, in AUTF8String header); */
    pub Save: unsafe extern "system" fn (this: *const nsIPersistentProperties, output: *const nsIOutputStream, header: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* nsISimpleEnumerator enumerate (); */
    pub Enumerate: unsafe extern "system" fn (this: *const nsIPersistentProperties, _retval: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult,

    /* AString getStringProperty (in AUTF8String key); */
    pub GetStringProperty: unsafe extern "system" fn (this: *const nsIPersistentProperties, key: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString setStringProperty (in AUTF8String key, in AString value); */
    pub SetStringProperty: unsafe extern "system" fn (this: *const nsIPersistentProperties, key: *const ::nsstring::nsACString, value: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPersistentProperties {

    /// ```text
    /// /**
    ///    * load a set of name/value pairs from the input stream
    ///    * names and values should be in UTF8
    ///    */
    /// ```
    ///

    /// `void load (in nsIInputStream input);`
    #[inline]
    pub unsafe fn Load(&self, input: *const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).Load)(self, input)
    }


    /// ```text
    /// /**
    ///    * output the values to the stream - results will be in UTF8
    ///    */
    /// ```
    ///

    /// `void save (in nsIOutputStream output, in AUTF8String header);`
    #[inline]
    pub unsafe fn Save(&self, output: *const nsIOutputStream, header: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Save)(self, output, header)
    }


    /// ```text
    /// /**
    ///    * get an enumeration of nsIPropertyElement objects,
    ///    * which are read-only (i.e. setting properties on the element will
        ///    * not make changes back into the source nsIPersistentProperties
        ///    */
        /// ```
        ///

        /// `nsISimpleEnumerator enumerate ();`
        #[inline]
        pub unsafe fn Enumerate(&self, _retval: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult {
            ((*self.vtable).Enumerate)(self, _retval)
        }


        /// ```text
        /// /**
        ///    * shortcut to nsIProperty's get() which retrieves a string value
        ///    * directly (and thus faster)
        ///    */
        /// ```
        ///

        /// `AString getStringProperty (in AUTF8String key);`
        #[inline]
        pub unsafe fn GetStringProperty(&self, key: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
            ((*self.vtable).GetStringProperty)(self, key, _retval)
        }


        /// ```text
        /// /**
        ///    * shortcut to nsIProperty's set() which sets a string value
        ///    * directly (and thus faster). If the given property already exists,
        ///    * then the old value will be returned
        ///    */
        /// ```
        ///

        /// `AString setStringProperty (in AUTF8String key, in AString value);`
        #[inline]
        pub unsafe fn SetStringProperty(&self, key: *const ::nsstring::nsACString, value: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
            ((*self.vtable).SetStringProperty)(self, key, value, _retval)
        }


    }


