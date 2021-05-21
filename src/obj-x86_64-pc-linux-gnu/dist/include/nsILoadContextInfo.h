/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsILoadContextInfo.idl
 */

#ifndef __gen_nsILoadContextInfo_h__
#define __gen_nsILoadContextInfo_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#include "mozilla/BasePrincipal.h"
class nsILoadContext; /* forward declaration */

class nsIDOMWindow; /* forward declaration */


/* starting interface:    nsILoadContextInfo */
#define NS_ILOADCONTEXTINFO_IID_STR "555e2f8a-a1f6-41dd-88ca-ed4ed6b98a22"

#define NS_ILOADCONTEXTINFO_IID \
  {0x555e2f8a, 0xa1f6, 0x41dd, \
    { 0x88, 0xca, 0xed, 0x4e, 0xd6, 0xb9, 0x8a, 0x22 }}

class nsILoadContextInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ILOADCONTEXTINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsILoadContextInfo;

  /* readonly attribute boolean isPrivate; */
  NS_IMETHOD GetIsPrivate(bool *aIsPrivate) = 0;

  /* readonly attribute boolean isAnonymous; */
  NS_IMETHOD GetIsAnonymous(bool *aIsAnonymous) = 0;

  /* [implicit_jscontext] readonly attribute jsval originAttributes; */
  NS_IMETHOD GetOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) = 0;

  /* [binaryname(OriginAttributesPtr),noscript,nostdcall,notxpcom] OriginAttributesNativePtr binaryOriginAttributesPtr (); */
  virtual const mozilla::OriginAttributes* OriginAttributesPtr(void) = 0;

   /**
   * De-XPCOMed getters
   */
  bool IsPrivate()
  {
    bool pb;
    GetIsPrivate(&pb);
    return pb;
  }
  bool IsAnonymous()
  {
    bool anon;
    GetIsAnonymous(&anon);
    return anon;
  }
  bool Equals(nsILoadContextInfo *aOther)
  {
    return IsAnonymous() == aOther->IsAnonymous() &&
           *OriginAttributesPtr() == *aOther->OriginAttributesPtr();
  }
};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsILoadContextInfo, NS_ILOADCONTEXTINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSILOADCONTEXTINFO \
  NS_IMETHOD GetIsPrivate(bool *aIsPrivate) override; \
  NS_IMETHOD GetIsAnonymous(bool *aIsAnonymous) override; \
  NS_IMETHOD GetOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) override; \
  virtual const mozilla::OriginAttributes* OriginAttributesPtr(void) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSILOADCONTEXTINFO \
  nsresult GetIsPrivate(bool *aIsPrivate); \
  nsresult GetIsAnonymous(bool *aIsAnonymous); \
  nsresult GetOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes); \
  const mozilla::OriginAttributes* OriginAttributesPtr(void); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSILOADCONTEXTINFO(_to) \
  NS_IMETHOD GetIsPrivate(bool *aIsPrivate) override { return _to GetIsPrivate(aIsPrivate); } \
  NS_IMETHOD GetIsAnonymous(bool *aIsAnonymous) override { return _to GetIsAnonymous(aIsAnonymous); } \
  NS_IMETHOD GetOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) override { return _to GetOriginAttributes(cx, aOriginAttributes); } \
  virtual const mozilla::OriginAttributes* OriginAttributesPtr(void) override { return _to OriginAttributesPtr(); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSILOADCONTEXTINFO(_to) \
  NS_IMETHOD GetIsPrivate(bool *aIsPrivate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsPrivate(aIsPrivate); } \
  NS_IMETHOD GetIsAnonymous(bool *aIsAnonymous) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsAnonymous(aIsAnonymous); } \
  NS_IMETHOD GetOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOriginAttributes(cx, aOriginAttributes); } \
  virtual const mozilla::OriginAttributes* OriginAttributesPtr(void) override; \


/* starting interface:    nsILoadContextInfoFactory */
#define NS_ILOADCONTEXTINFOFACTORY_IID_STR "c1c7023d-4318-4f99-8307-b5ccf0558793"

#define NS_ILOADCONTEXTINFOFACTORY_IID \
  {0xc1c7023d, 0x4318, 0x4f99, \
    { 0x83, 0x07, 0xb5, 0xcc, 0xf0, 0x55, 0x87, 0x93 }}

class NS_NO_VTABLE nsILoadContextInfoFactory : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ILOADCONTEXTINFOFACTORY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsILoadContextInfoFactory;

  /* readonly attribute nsILoadContextInfo default; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDefault(nsILoadContextInfo **aDefault) = 0;

  /* readonly attribute nsILoadContextInfo private; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPrivate(nsILoadContextInfo **aPrivate) = 0;

  /* readonly attribute nsILoadContextInfo anonymous; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAnonymous(nsILoadContextInfo **aAnonymous) = 0;

  /* [implicit_jscontext] nsILoadContextInfo custom (in boolean aAnonymous, in jsval aOriginAttributes); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Custom(bool aAnonymous, JS::HandleValue aOriginAttributes, JSContext* cx, nsILoadContextInfo **_retval) = 0;

  /* nsILoadContextInfo fromLoadContext (in nsILoadContext aLoadContext, in boolean aAnonymous); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FromLoadContext(nsILoadContext *aLoadContext, bool aAnonymous, nsILoadContextInfo **_retval) = 0;

  /* nsILoadContextInfo fromWindow (in nsIDOMWindow aWindow, in boolean aAnonymous); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FromWindow(nsIDOMWindow *aWindow, bool aAnonymous, nsILoadContextInfo **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsILoadContextInfoFactory, NS_ILOADCONTEXTINFOFACTORY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSILOADCONTEXTINFOFACTORY \
  NS_IMETHOD GetDefault(nsILoadContextInfo **aDefault) override; \
  NS_IMETHOD GetPrivate(nsILoadContextInfo **aPrivate) override; \
  NS_IMETHOD GetAnonymous(nsILoadContextInfo **aAnonymous) override; \
  NS_IMETHOD Custom(bool aAnonymous, JS::HandleValue aOriginAttributes, JSContext* cx, nsILoadContextInfo **_retval) override; \
  NS_IMETHOD FromLoadContext(nsILoadContext *aLoadContext, bool aAnonymous, nsILoadContextInfo **_retval) override; \
  NS_IMETHOD FromWindow(nsIDOMWindow *aWindow, bool aAnonymous, nsILoadContextInfo **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSILOADCONTEXTINFOFACTORY \
  nsresult GetDefault(nsILoadContextInfo **aDefault); \
  nsresult GetPrivate(nsILoadContextInfo **aPrivate); \
  nsresult GetAnonymous(nsILoadContextInfo **aAnonymous); \
  nsresult Custom(bool aAnonymous, JS::HandleValue aOriginAttributes, JSContext* cx, nsILoadContextInfo **_retval); \
  nsresult FromLoadContext(nsILoadContext *aLoadContext, bool aAnonymous, nsILoadContextInfo **_retval); \
  nsresult FromWindow(nsIDOMWindow *aWindow, bool aAnonymous, nsILoadContextInfo **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSILOADCONTEXTINFOFACTORY(_to) \
  NS_IMETHOD GetDefault(nsILoadContextInfo **aDefault) override { return _to GetDefault(aDefault); } \
  NS_IMETHOD GetPrivate(nsILoadContextInfo **aPrivate) override { return _to GetPrivate(aPrivate); } \
  NS_IMETHOD GetAnonymous(nsILoadContextInfo **aAnonymous) override { return _to GetAnonymous(aAnonymous); } \
  NS_IMETHOD Custom(bool aAnonymous, JS::HandleValue aOriginAttributes, JSContext* cx, nsILoadContextInfo **_retval) override { return _to Custom(aAnonymous, aOriginAttributes, cx, _retval); } \
  NS_IMETHOD FromLoadContext(nsILoadContext *aLoadContext, bool aAnonymous, nsILoadContextInfo **_retval) override { return _to FromLoadContext(aLoadContext, aAnonymous, _retval); } \
  NS_IMETHOD FromWindow(nsIDOMWindow *aWindow, bool aAnonymous, nsILoadContextInfo **_retval) override { return _to FromWindow(aWindow, aAnonymous, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSILOADCONTEXTINFOFACTORY(_to) \
  NS_IMETHOD GetDefault(nsILoadContextInfo **aDefault) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDefault(aDefault); } \
  NS_IMETHOD GetPrivate(nsILoadContextInfo **aPrivate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrivate(aPrivate); } \
  NS_IMETHOD GetAnonymous(nsILoadContextInfo **aAnonymous) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAnonymous(aAnonymous); } \
  NS_IMETHOD Custom(bool aAnonymous, JS::HandleValue aOriginAttributes, JSContext* cx, nsILoadContextInfo **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Custom(aAnonymous, aOriginAttributes, cx, _retval); } \
  NS_IMETHOD FromLoadContext(nsILoadContext *aLoadContext, bool aAnonymous, nsILoadContextInfo **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FromLoadContext(aLoadContext, aAnonymous, _retval); } \
  NS_IMETHOD FromWindow(nsIDOMWindow *aWindow, bool aAnonymous, nsILoadContextInfo **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FromWindow(aWindow, aAnonymous, _retval); } 


#endif /* __gen_nsILoadContextInfo_h__ */
