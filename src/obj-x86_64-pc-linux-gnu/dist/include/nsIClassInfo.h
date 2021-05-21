/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/components/nsIClassInfo.idl
 */

#ifndef __gen_nsIClassInfo_h__
#define __gen_nsIClassInfo_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIXPCScriptable; /* forward declaration */


/* starting interface:    nsIClassInfo */
#define NS_ICLASSINFO_IID_STR "a60569d7-d401-4677-ba63-2aa5971af25d"

#define NS_ICLASSINFO_IID \
  {0xa60569d7, 0xd401, 0x4677, \
    { 0xba, 0x63, 0x2a, 0xa5, 0x97, 0x1a, 0xf2, 0x5d }}

class NS_NO_VTABLE nsIClassInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICLASSINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIClassInfo;

  /* readonly attribute Array<nsIIDRef> interfaces; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetInterfaces(nsTArray<nsIID>& aInterfaces) = 0;

  /* nsIXPCScriptable getScriptableHelper (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetScriptableHelper(nsIXPCScriptable **_retval) = 0;

  /* readonly attribute AUTF8String contractID; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetContractID(nsACString& aContractID) = 0;

  /* readonly attribute AUTF8String classDescription; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetClassDescription(nsACString& aClassDescription) = 0;

  /* readonly attribute nsCIDPtr classID; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetClassID(nsCID * * aClassID) = 0;

  enum {
    SINGLETON = 1U,
    THREADSAFE = 2U,
    MAIN_THREAD_ONLY = 4U,
    DOM_OBJECT = 8U,
    PLUGIN_OBJECT = 16U,
    SINGLETON_CLASSINFO = 32U,
    RESERVED = 2147483648U
  };

  /* readonly attribute uint32_t flags; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFlags(uint32_t *aFlags) = 0;

  /* [noscript] readonly attribute nsCID classIDNoAlloc; */
  NS_IMETHOD GetClassIDNoAlloc(nsCID * aClassIDNoAlloc) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIClassInfo, NS_ICLASSINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICLASSINFO \
  NS_IMETHOD GetInterfaces(nsTArray<nsIID>& aInterfaces) override; \
  NS_IMETHOD GetScriptableHelper(nsIXPCScriptable **_retval) override; \
  NS_IMETHOD GetContractID(nsACString& aContractID) override; \
  NS_IMETHOD GetClassDescription(nsACString& aClassDescription) override; \
  NS_IMETHOD GetClassID(nsCID * * aClassID) override; \
  NS_IMETHOD GetFlags(uint32_t *aFlags) override; \
  NS_IMETHOD GetClassIDNoAlloc(nsCID * aClassIDNoAlloc) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICLASSINFO \
  nsresult GetInterfaces(nsTArray<nsIID>& aInterfaces); \
  nsresult GetScriptableHelper(nsIXPCScriptable **_retval); \
  nsresult GetContractID(nsACString& aContractID); \
  nsresult GetClassDescription(nsACString& aClassDescription); \
  nsresult GetClassID(nsCID * * aClassID); \
  nsresult GetFlags(uint32_t *aFlags); \
  nsresult GetClassIDNoAlloc(nsCID * aClassIDNoAlloc); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICLASSINFO(_to) \
  NS_IMETHOD GetInterfaces(nsTArray<nsIID>& aInterfaces) override { return _to GetInterfaces(aInterfaces); } \
  NS_IMETHOD GetScriptableHelper(nsIXPCScriptable **_retval) override { return _to GetScriptableHelper(_retval); } \
  NS_IMETHOD GetContractID(nsACString& aContractID) override { return _to GetContractID(aContractID); } \
  NS_IMETHOD GetClassDescription(nsACString& aClassDescription) override { return _to GetClassDescription(aClassDescription); } \
  NS_IMETHOD GetClassID(nsCID * * aClassID) override { return _to GetClassID(aClassID); } \
  NS_IMETHOD GetFlags(uint32_t *aFlags) override { return _to GetFlags(aFlags); } \
  NS_IMETHOD GetClassIDNoAlloc(nsCID * aClassIDNoAlloc) override { return _to GetClassIDNoAlloc(aClassIDNoAlloc); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICLASSINFO(_to) \
  NS_IMETHOD GetInterfaces(nsTArray<nsIID>& aInterfaces) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInterfaces(aInterfaces); } \
  NS_IMETHOD GetScriptableHelper(nsIXPCScriptable **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScriptableHelper(_retval); } \
  NS_IMETHOD GetContractID(nsACString& aContractID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContractID(aContractID); } \
  NS_IMETHOD GetClassDescription(nsACString& aClassDescription) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetClassDescription(aClassDescription); } \
  NS_IMETHOD GetClassID(nsCID * * aClassID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetClassID(aClassID); } \
  NS_IMETHOD GetFlags(uint32_t *aFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFlags(aFlags); } \
  NS_IMETHOD GetClassIDNoAlloc(nsCID * aClassIDNoAlloc) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetClassIDNoAlloc(aClassIDNoAlloc); } 


#endif /* __gen_nsIClassInfo_h__ */
