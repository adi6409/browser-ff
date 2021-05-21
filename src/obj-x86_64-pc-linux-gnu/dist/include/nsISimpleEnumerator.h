/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsISimpleEnumerator.idl
 */

#ifndef __gen_nsISimpleEnumerator_h__
#define __gen_nsISimpleEnumerator_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIJSEnumerator */
#define NS_IJSENUMERATOR_IID_STR "4432e8ae-d4d3-42a6-a4d1-829f1c29512b"

#define NS_IJSENUMERATOR_IID \
  {0x4432e8ae, 0xd4d3, 0x42a6, \
    { 0xa4, 0xd1, 0x82, 0x9f, 0x1c, 0x29, 0x51, 0x2b }}

class NS_NO_VTABLE nsIJSEnumerator : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IJSENUMERATOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIJSEnumerator;

  /* [symbol] nsIJSEnumerator iterator (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Iterator(nsIJSEnumerator **_retval) = 0;

  /* [implicit_jscontext] jsval next (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Next(JSContext* cx, JS::MutableHandleValue _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIJSEnumerator, NS_IJSENUMERATOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIJSENUMERATOR \
  NS_IMETHOD Iterator(nsIJSEnumerator **_retval) override; \
  NS_IMETHOD Next(JSContext* cx, JS::MutableHandleValue _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIJSENUMERATOR \
  nsresult Iterator(nsIJSEnumerator **_retval); \
  nsresult Next(JSContext* cx, JS::MutableHandleValue _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIJSENUMERATOR(_to) \
  NS_IMETHOD Iterator(nsIJSEnumerator **_retval) override { return _to Iterator(_retval); } \
  NS_IMETHOD Next(JSContext* cx, JS::MutableHandleValue _retval) override { return _to Next(cx, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIJSENUMERATOR(_to) \
  NS_IMETHOD Iterator(nsIJSEnumerator **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Iterator(_retval); } \
  NS_IMETHOD Next(JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Next(cx, _retval); } 


/* starting interface:    nsISimpleEnumeratorBase */
#define NS_ISIMPLEENUMERATORBASE_IID_STR "796f340d-0a2a-490b-9c60-640765e99782"

#define NS_ISIMPLEENUMERATORBASE_IID \
  {0x796f340d, 0x0a2a, 0x490b, \
    { 0x9c, 0x60, 0x64, 0x07, 0x65, 0xe9, 0x97, 0x82 }}

class NS_NO_VTABLE nsISimpleEnumeratorBase : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISIMPLEENUMERATORBASE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISimpleEnumeratorBase;

  /* [symbol] nsIJSEnumerator iterator (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Iterator(nsIJSEnumerator **_retval) = 0;

  /* nsIJSEnumerator entries (in nsIIDRef aIface); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Entries(const nsIID & aIface, nsIJSEnumerator **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISimpleEnumeratorBase, NS_ISIMPLEENUMERATORBASE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISIMPLEENUMERATORBASE \
  NS_IMETHOD Iterator(nsIJSEnumerator **_retval) override; \
  NS_IMETHOD Entries(const nsIID & aIface, nsIJSEnumerator **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISIMPLEENUMERATORBASE \
  nsresult Iterator(nsIJSEnumerator **_retval); \
  nsresult Entries(const nsIID & aIface, nsIJSEnumerator **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISIMPLEENUMERATORBASE(_to) \
  NS_IMETHOD Iterator(nsIJSEnumerator **_retval) override { return _to Iterator(_retval); } \
  NS_IMETHOD Entries(const nsIID & aIface, nsIJSEnumerator **_retval) override { return _to Entries(aIface, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISIMPLEENUMERATORBASE(_to) \
  NS_IMETHOD Iterator(nsIJSEnumerator **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Iterator(_retval); } \
  NS_IMETHOD Entries(const nsIID & aIface, nsIJSEnumerator **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Entries(aIface, _retval); } 


/* starting interface:    nsISimpleEnumerator */
#define NS_ISIMPLEENUMERATOR_IID_STR "d1899240-f9d2-11d2-bdd6-000064657374"

#define NS_ISIMPLEENUMERATOR_IID \
  {0xd1899240, 0xf9d2, 0x11d2, \
    { 0xbd, 0xd6, 0x00, 0x00, 0x64, 0x65, 0x73, 0x74 }}

class NS_NO_VTABLE nsISimpleEnumerator : public nsISimpleEnumeratorBase {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISIMPLEENUMERATOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISimpleEnumerator;

  /* boolean hasMoreElements (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HasMoreElements(bool *_retval) = 0;

  /* nsISupports getNext (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNext(nsISupports **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISimpleEnumerator, NS_ISIMPLEENUMERATOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISIMPLEENUMERATOR \
  NS_IMETHOD HasMoreElements(bool *_retval) override; \
  NS_IMETHOD GetNext(nsISupports **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISIMPLEENUMERATOR \
  nsresult HasMoreElements(bool *_retval); \
  nsresult GetNext(nsISupports **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISIMPLEENUMERATOR(_to) \
  NS_IMETHOD HasMoreElements(bool *_retval) override { return _to HasMoreElements(_retval); } \
  NS_IMETHOD GetNext(nsISupports **_retval) override { return _to GetNext(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISIMPLEENUMERATOR(_to) \
  NS_IMETHOD HasMoreElements(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasMoreElements(_retval); } \
  NS_IMETHOD GetNext(nsISupports **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNext(_retval); } 


#endif /* __gen_nsISimpleEnumerator_h__ */
