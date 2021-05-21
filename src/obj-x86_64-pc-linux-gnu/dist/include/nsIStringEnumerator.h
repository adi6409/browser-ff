/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIStringEnumerator.idl
 */

#ifndef __gen_nsIStringEnumerator_h__
#define __gen_nsIStringEnumerator_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIJSEnumerator; /* forward declaration */


/* starting interface:    nsIStringEnumeratorBase */
#define NS_ISTRINGENUMERATORBASE_IID_STR "f5213d15-a4d1-4fb7-8a48-d69ccb7fb0eb"

#define NS_ISTRINGENUMERATORBASE_IID \
  {0xf5213d15, 0xa4d1, 0x4fb7, \
    { 0x8a, 0x48, 0xd6, 0x9c, 0xcb, 0x7f, 0xb0, 0xeb }}

class NS_NO_VTABLE nsIStringEnumeratorBase : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISTRINGENUMERATORBASE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIStringEnumeratorBase;

  /* [binaryname(StringIterator),symbol] nsIJSEnumerator iterator (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD StringIterator(nsIJSEnumerator **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIStringEnumeratorBase, NS_ISTRINGENUMERATORBASE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISTRINGENUMERATORBASE \
  NS_IMETHOD StringIterator(nsIJSEnumerator **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISTRINGENUMERATORBASE \
  nsresult StringIterator(nsIJSEnumerator **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISTRINGENUMERATORBASE(_to) \
  NS_IMETHOD StringIterator(nsIJSEnumerator **_retval) override { return _to StringIterator(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISTRINGENUMERATORBASE(_to) \
  NS_IMETHOD StringIterator(nsIJSEnumerator **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StringIterator(_retval); } 


/* starting interface:    nsIStringEnumerator */
#define NS_ISTRINGENUMERATOR_IID_STR "50d3ef6c-9380-4f06-9fb2-95488f7d141c"

#define NS_ISTRINGENUMERATOR_IID \
  {0x50d3ef6c, 0x9380, 0x4f06, \
    { 0x9f, 0xb2, 0x95, 0x48, 0x8f, 0x7d, 0x14, 0x1c }}

class NS_NO_VTABLE nsIStringEnumerator : public nsIStringEnumeratorBase {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISTRINGENUMERATOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIStringEnumerator;

  /* boolean hasMore (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HasMore(bool *_retval) = 0;

  /* AString getNext (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNext(nsAString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIStringEnumerator, NS_ISTRINGENUMERATOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISTRINGENUMERATOR \
  NS_IMETHOD HasMore(bool *_retval) override; \
  NS_IMETHOD GetNext(nsAString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISTRINGENUMERATOR \
  nsresult HasMore(bool *_retval); \
  nsresult GetNext(nsAString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISTRINGENUMERATOR(_to) \
  NS_IMETHOD HasMore(bool *_retval) override { return _to HasMore(_retval); } \
  NS_IMETHOD GetNext(nsAString& _retval) override { return _to GetNext(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISTRINGENUMERATOR(_to) \
  NS_IMETHOD HasMore(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasMore(_retval); } \
  NS_IMETHOD GetNext(nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNext(_retval); } 


/* starting interface:    nsIUTF8StringEnumerator */
#define NS_IUTF8STRINGENUMERATOR_IID_STR "9bdf1010-3695-4907-95ed-83d0410ec307"

#define NS_IUTF8STRINGENUMERATOR_IID \
  {0x9bdf1010, 0x3695, 0x4907, \
    { 0x95, 0xed, 0x83, 0xd0, 0x41, 0x0e, 0xc3, 0x07 }}

class NS_NO_VTABLE nsIUTF8StringEnumerator : public nsIStringEnumeratorBase {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IUTF8STRINGENUMERATOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUTF8StringEnumerator;

  /* boolean hasMore (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HasMore(bool *_retval) = 0;

  /* AUTF8String getNext (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNext(nsACString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUTF8StringEnumerator, NS_IUTF8STRINGENUMERATOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIUTF8STRINGENUMERATOR \
  NS_IMETHOD HasMore(bool *_retval) override; \
  NS_IMETHOD GetNext(nsACString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIUTF8STRINGENUMERATOR \
  nsresult HasMore(bool *_retval); \
  nsresult GetNext(nsACString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIUTF8STRINGENUMERATOR(_to) \
  NS_IMETHOD HasMore(bool *_retval) override { return _to HasMore(_retval); } \
  NS_IMETHOD GetNext(nsACString& _retval) override { return _to GetNext(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIUTF8STRINGENUMERATOR(_to) \
  NS_IMETHOD HasMore(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasMore(_retval); } \
  NS_IMETHOD GetNext(nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNext(_retval); } 


#endif /* __gen_nsIStringEnumerator_h__ */
