/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/processtools/nsIProcessToolsService.idl
 */

#ifndef __gen_nsIProcessToolsService_h__
#define __gen_nsIProcessToolsService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIProcessToolsService */
#define NS_IPROCESSTOOLSSERVICE_IID_STR "1341f571-ebed-4305-b264-4d8fc3b6b11c"

#define NS_IPROCESSTOOLSSERVICE_IID \
  {0x1341f571, 0xebed, 0x4305, \
    { 0xb2, 0x64, 0x4d, 0x8f, 0xc3, 0xb6, 0xb1, 0x1c }}

class NS_NO_VTABLE nsIProcessToolsService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPROCESSTOOLSSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIProcessToolsService;

  /* void kill (in unsigned long long pid); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Kill(uint64_t pid) = 0;

  /* readonly attribute unsigned long long pid; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPid(uint64_t *aPid) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIProcessToolsService, NS_IPROCESSTOOLSSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPROCESSTOOLSSERVICE \
  NS_IMETHOD Kill(uint64_t pid) override; \
  NS_IMETHOD GetPid(uint64_t *aPid) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPROCESSTOOLSSERVICE \
  nsresult Kill(uint64_t pid); \
  nsresult GetPid(uint64_t *aPid); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPROCESSTOOLSSERVICE(_to) \
  NS_IMETHOD Kill(uint64_t pid) override { return _to Kill(pid); } \
  NS_IMETHOD GetPid(uint64_t *aPid) override { return _to GetPid(aPid); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPROCESSTOOLSSERVICE(_to) \
  NS_IMETHOD Kill(uint64_t pid) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Kill(pid); } \
  NS_IMETHOD GetPid(uint64_t *aPid) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPid(aPid); } 


#endif /* __gen_nsIProcessToolsService_h__ */
