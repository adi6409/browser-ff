/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/threads/nsIEnvironment.idl
 */

#ifndef __gen_nsIEnvironment_h__
#define __gen_nsIEnvironment_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIEnvironment */
#define NS_IENVIRONMENT_IID_STR "101d5941-d820-4e85-a266-9a3469940807"

#define NS_IENVIRONMENT_IID \
  {0x101d5941, 0xd820, 0x4e85, \
    { 0xa2, 0x66, 0x9a, 0x34, 0x69, 0x94, 0x08, 0x07 }}

class NS_NO_VTABLE nsIEnvironment : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IENVIRONMENT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIEnvironment;

  /* void set (in AString aName, in AString aValue); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Set(const nsAString& aName, const nsAString& aValue) = 0;

  /* AString get (in AString aName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Get(const nsAString& aName, nsAString& _retval) = 0;

  /* boolean exists (in AString aName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Exists(const nsAString& aName, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIEnvironment, NS_IENVIRONMENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIENVIRONMENT \
  NS_IMETHOD Set(const nsAString& aName, const nsAString& aValue) override; \
  NS_IMETHOD Get(const nsAString& aName, nsAString& _retval) override; \
  NS_IMETHOD Exists(const nsAString& aName, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIENVIRONMENT \
  nsresult Set(const nsAString& aName, const nsAString& aValue); \
  nsresult Get(const nsAString& aName, nsAString& _retval); \
  nsresult Exists(const nsAString& aName, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIENVIRONMENT(_to) \
  NS_IMETHOD Set(const nsAString& aName, const nsAString& aValue) override { return _to Set(aName, aValue); } \
  NS_IMETHOD Get(const nsAString& aName, nsAString& _retval) override { return _to Get(aName, _retval); } \
  NS_IMETHOD Exists(const nsAString& aName, bool *_retval) override { return _to Exists(aName, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIENVIRONMENT(_to) \
  NS_IMETHOD Set(const nsAString& aName, const nsAString& aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Set(aName, aValue); } \
  NS_IMETHOD Get(const nsAString& aName, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Get(aName, _retval); } \
  NS_IMETHOD Exists(const nsAString& aName, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Exists(aName, _retval); } 


#endif /* __gen_nsIEnvironment_h__ */
