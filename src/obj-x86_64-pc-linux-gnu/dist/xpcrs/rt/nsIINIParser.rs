//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIINIParser.idl
//


/// `interface nsIINIParser : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIINIParser {
    vtable: *const nsIINIParserVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIINIParser.
unsafe impl XpCom for nsIINIParser {
    const IID: nsIID = nsID(0x7eb955f6, 0x3e78, 0x4d39,
        [0xb7, 0x2f, 0xc1, 0xbf, 0x12, 0xa9, 0x4b, 0xce]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIINIParser {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIINIParser.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIINIParserCoerce {
    /// Cheaply cast a value of this type from a `nsIINIParser`.
    fn coerce_from(v: &nsIINIParser) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIINIParserCoerce for nsIINIParser {
    #[inline]
    fn coerce_from(v: &nsIINIParser) -> &Self {
        v
    }
}

impl nsIINIParser {
    /// Cast this `nsIINIParser` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIINIParserCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIINIParser {
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
impl<T: nsISupportsCoerce> nsIINIParserCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIINIParser) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIINIParser
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIINIParserVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIUTF8StringEnumerator getSections (); */
    pub GetSections: unsafe extern "system" fn (this: *const nsIINIParser, _retval: *mut*const nsIUTF8StringEnumerator) -> ::nserror::nsresult,

    /* nsIUTF8StringEnumerator getKeys (in AUTF8String aSection); */
    pub GetKeys: unsafe extern "system" fn (this: *const nsIINIParser, aSection: *const ::nsstring::nsACString, _retval: *mut*const nsIUTF8StringEnumerator) -> ::nserror::nsresult,

    /* AUTF8String getString (in AUTF8String aSection, in AUTF8String aKey); */
    pub GetString: unsafe extern "system" fn (this: *const nsIINIParser, aSection: *const ::nsstring::nsACString, aKey: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIINIParser {

    /// ```text
    /// /**
    ///    * Enumerates the [section]s available in the INI file.
    ///    */
    /// ```
    ///

    /// `nsIUTF8StringEnumerator getSections ();`
    #[inline]
    pub unsafe fn GetSections(&self, _retval: *mut*const nsIUTF8StringEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).GetSections)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Enumerates the keys available within a section.
    ///    */
    /// ```
    ///

    /// `nsIUTF8StringEnumerator getKeys (in AUTF8String aSection);`
    #[inline]
    pub unsafe fn GetKeys(&self, aSection: *const ::nsstring::nsACString, _retval: *mut*const nsIUTF8StringEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).GetKeys)(self, aSection, _retval)
    }


    /// ```text
    /// /**
    ///    * Get the value of a string for a particular section and key.
    ///    */
    /// ```
    ///

    /// `AUTF8String getString (in AUTF8String aSection, in AUTF8String aKey);`
    #[inline]
    pub unsafe fn GetString(&self, aSection: *const ::nsstring::nsACString, aKey: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetString)(self, aSection, aKey, _retval)
    }


}


/// `interface nsIINIParserWriter : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIINIParserWriter {
    vtable: *const nsIINIParserWriterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIINIParserWriter.
unsafe impl XpCom for nsIINIParserWriter {
    const IID: nsIID = nsID(0xb67bb24b, 0x31a3, 0x4a6a,
        [0xa5, 0xd9, 0x04, 0x85, 0xc9, 0xaf, 0x5a, 0x04]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIINIParserWriter {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIINIParserWriter.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIINIParserWriterCoerce {
    /// Cheaply cast a value of this type from a `nsIINIParserWriter`.
    fn coerce_from(v: &nsIINIParserWriter) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIINIParserWriterCoerce for nsIINIParserWriter {
    #[inline]
    fn coerce_from(v: &nsIINIParserWriter) -> &Self {
        v
    }
}

impl nsIINIParserWriter {
    /// Cast this `nsIINIParserWriter` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIINIParserWriterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIINIParserWriter {
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
impl<T: nsISupportsCoerce> nsIINIParserWriterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIINIParserWriter) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIINIParserWriter
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIINIParserWriterVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void setString (in AUTF8String aSection, in AUTF8String aKey, in AUTF8String aValue); */
    pub SetString: unsafe extern "system" fn (this: *const nsIINIParserWriter, aSection: *const ::nsstring::nsACString, aKey: *const ::nsstring::nsACString, aValue: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void writeFile (in nsIFile aINIFile); */
    pub WriteFile: unsafe extern "system" fn (this: *const nsIINIParserWriter, aINIFile: *const nsIFile) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIINIParserWriter {

    /// ```text
    /// /**
    ///    * Set the value of a string for a particular section and key.
    ///    */
    /// ```
    ///

    /// `void setString (in AUTF8String aSection, in AUTF8String aKey, in AUTF8String aValue);`
    #[inline]
    pub unsafe fn SetString(&self, aSection: *const ::nsstring::nsACString, aKey: *const ::nsstring::nsACString, aValue: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetString)(self, aSection, aKey, aValue)
    }


    /// ```text
    /// /**
    ///    * Write to the INI file.
    ///    */
    /// ```
    ///

    /// `void writeFile (in nsIFile aINIFile);`
    #[inline]
    pub unsafe fn WriteFile(&self, aINIFile: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).WriteFile)(self, aINIFile)
    }


}


/// `interface nsIINIParserFactory : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIINIParserFactory {
    vtable: *const nsIINIParserFactoryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIINIParserFactory.
unsafe impl XpCom for nsIINIParserFactory {
    const IID: nsIID = nsID(0xccae7ea5, 0x1218, 0x4b51,
        [0xae, 0xcb, 0xc2, 0xd8, 0xec, 0xd4, 0x6a, 0xf9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIINIParserFactory {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIINIParserFactory.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIINIParserFactoryCoerce {
    /// Cheaply cast a value of this type from a `nsIINIParserFactory`.
    fn coerce_from(v: &nsIINIParserFactory) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIINIParserFactoryCoerce for nsIINIParserFactory {
    #[inline]
    fn coerce_from(v: &nsIINIParserFactory) -> &Self {
        v
    }
}

impl nsIINIParserFactory {
    /// Cast this `nsIINIParserFactory` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIINIParserFactoryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIINIParserFactory {
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
impl<T: nsISupportsCoerce> nsIINIParserFactoryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIINIParserFactory) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIINIParserFactory
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIINIParserFactoryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIINIParser createINIParser ([optional] in nsIFile aINIFile); */
    pub CreateINIParser: unsafe extern "system" fn (this: *const nsIINIParserFactory, aINIFile: *const nsIFile, _retval: *mut *const nsIINIParser) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIINIParserFactory {

    /// ```text
    /// /**
    ///    * Create an iniparser instance from a local file.
    ///    */
    /// ```
    ///

    /// `nsIINIParser createINIParser ([optional] in nsIFile aINIFile);`
    #[inline]
    pub unsafe fn CreateINIParser(&self, aINIFile: *const nsIFile, _retval: *mut *const nsIINIParser) -> ::nserror::nsresult {
        ((*self.vtable).CreateINIParser)(self, aINIFile, _retval)
    }


}


