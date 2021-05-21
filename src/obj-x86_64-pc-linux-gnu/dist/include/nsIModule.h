/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/components/nsIModule.idl
 */

#ifndef __gen_nsIModule_h__
#define __gen_nsIModule_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIFile; /* forward declaration */

class nsIComponentManager; /* forward declaration */


/* starting interface:    nsIModule */
#define NS_IMODULE_IID_STR "7392d032-5371-11d3-994e-00805fd26fee"

#define NS_IMODULE_IID \
  {0x7392d032, 0x5371, 0x11d3, \
    { 0x99, 0x4e, 0x00, 0x80, 0x5f, 0xd2, 0x6f, 0xee }}

class NS_NO_VTABLE nsIModule : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IMODULE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIModule;

  /* void getClassObject (in nsIComponentManager aCompMgr, in nsCIDRef aClass, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult aResult); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetClassObject(nsIComponentManager *aCompMgr, const nsCID & aClass, const nsIID & aIID, void * * aResult) = 0;

  /* void registerSelf (in nsIComponentManager aCompMgr, in nsIFile aLocation, in string aLoaderStr, in string aType); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RegisterSelf(nsIComponentManager *aCompMgr, nsIFile *aLocation, const char * aLoaderStr, const char * aType) = 0;

  /* void unregisterSelf (in nsIComponentManager aCompMgr, in nsIFile aLocation, in string aLoaderStr); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UnregisterSelf(nsIComponentManager *aCompMgr, nsIFile *aLocation, const char * aLoaderStr) = 0;

  /* boolean canUnload (in nsIComponentManager aCompMgr); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CanUnload(nsIComponentManager *aCompMgr, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIModule, NS_IMODULE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIMODULE \
  NS_IMETHOD GetClassObject(nsIComponentManager *aCompMgr, const nsCID & aClass, const nsIID & aIID, void * * aResult) override; \
  NS_IMETHOD RegisterSelf(nsIComponentManager *aCompMgr, nsIFile *aLocation, const char * aLoaderStr, const char * aType) override; \
  NS_IMETHOD UnregisterSelf(nsIComponentManager *aCompMgr, nsIFile *aLocation, const char * aLoaderStr) override; \
  NS_IMETHOD CanUnload(nsIComponentManager *aCompMgr, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIMODULE \
  nsresult GetClassObject(nsIComponentManager *aCompMgr, const nsCID & aClass, const nsIID & aIID, void * * aResult); \
  nsresult RegisterSelf(nsIComponentManager *aCompMgr, nsIFile *aLocation, const char * aLoaderStr, const char * aType); \
  nsresult UnregisterSelf(nsIComponentManager *aCompMgr, nsIFile *aLocation, const char * aLoaderStr); \
  nsresult CanUnload(nsIComponentManager *aCompMgr, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIMODULE(_to) \
  NS_IMETHOD GetClassObject(nsIComponentManager *aCompMgr, const nsCID & aClass, const nsIID & aIID, void * * aResult) override { return _to GetClassObject(aCompMgr, aClass, aIID, aResult); } \
  NS_IMETHOD RegisterSelf(nsIComponentManager *aCompMgr, nsIFile *aLocation, const char * aLoaderStr, const char * aType) override { return _to RegisterSelf(aCompMgr, aLocation, aLoaderStr, aType); } \
  NS_IMETHOD UnregisterSelf(nsIComponentManager *aCompMgr, nsIFile *aLocation, const char * aLoaderStr) override { return _to UnregisterSelf(aCompMgr, aLocation, aLoaderStr); } \
  NS_IMETHOD CanUnload(nsIComponentManager *aCompMgr, bool *_retval) override { return _to CanUnload(aCompMgr, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIMODULE(_to) \
  NS_IMETHOD GetClassObject(nsIComponentManager *aCompMgr, const nsCID & aClass, const nsIID & aIID, void * * aResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetClassObject(aCompMgr, aClass, aIID, aResult); } \
  NS_IMETHOD RegisterSelf(nsIComponentManager *aCompMgr, nsIFile *aLocation, const char * aLoaderStr, const char * aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterSelf(aCompMgr, aLocation, aLoaderStr, aType); } \
  NS_IMETHOD UnregisterSelf(nsIComponentManager *aCompMgr, nsIFile *aLocation, const char * aLoaderStr) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnregisterSelf(aCompMgr, aLocation, aLoaderStr); } \
  NS_IMETHOD CanUnload(nsIComponentManager *aCompMgr, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CanUnload(aCompMgr, _retval); } 


#endif /* __gen_nsIModule_h__ */
