/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/tests/NotXPCOMTest.idl
 */

#ifndef __gen_NotXPCOMTest_h__
#define __gen_NotXPCOMTest_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIScriptableOK */
#define NS_ISCRIPTABLEOK_IID_STR "93142a4f-e4cf-424a-b833-e638f87d2607"

#define NS_ISCRIPTABLEOK_IID \
  {0x93142a4f, 0xe4cf, 0x424a, \
    { 0xb8, 0x33, 0xe6, 0x38, 0xf8, 0x7d, 0x26, 0x07 }}

class NS_NO_VTABLE nsIScriptableOK : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISCRIPTABLEOK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIScriptableOK;

  /* void method1 (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Method1(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIScriptableOK, NS_ISCRIPTABLEOK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISCRIPTABLEOK \
  NS_IMETHOD Method1(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISCRIPTABLEOK \
  nsresult Method1(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISCRIPTABLEOK(_to) \
  NS_IMETHOD Method1(void) override { return _to Method1(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISCRIPTABLEOK(_to) \
  NS_IMETHOD Method1(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Method1(); } 


/* starting interface:    nsIScriptableWithNotXPCOM */
#define NS_ISCRIPTABLEWITHNOTXPCOM_IID_STR "237d01a3-771e-4c6e-adf9-c97f9aab2950"

#define NS_ISCRIPTABLEWITHNOTXPCOM_IID \
  {0x237d01a3, 0x771e, 0x4c6e, \
    { 0xad, 0xf9, 0xc9, 0x7f, 0x9a, 0xab, 0x29, 0x50 }}

class NS_NO_VTABLE nsIScriptableWithNotXPCOM : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISCRIPTABLEWITHNOTXPCOM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIScriptableWithNotXPCOM;

  /* [notxpcom] void method2 (); */
  NS_IMETHOD_(void) Method2(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIScriptableWithNotXPCOM, NS_ISCRIPTABLEWITHNOTXPCOM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISCRIPTABLEWITHNOTXPCOM \
  NS_IMETHOD_(void) Method2(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISCRIPTABLEWITHNOTXPCOM \
  nsresult_(void) Method2(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISCRIPTABLEWITHNOTXPCOM(_to) \
  NS_IMETHOD_(void) Method2(void) override { return _to Method2(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISCRIPTABLEWITHNOTXPCOM(_to) \
  NS_IMETHOD_(void) Method2(void) override; 


#endif /* __gen_NotXPCOMTest_h__ */
