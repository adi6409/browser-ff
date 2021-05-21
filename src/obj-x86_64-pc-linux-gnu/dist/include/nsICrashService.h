/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/crashes/nsICrashService.idl
 */

#ifndef __gen_nsICrashService_h__
#define __gen_nsICrashService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsICrashService */
#define NS_ICRASHSERVICE_IID_STR "70bd93ff-88fa-4600-8af8-57c8d002dbac"

#define NS_ICRASHSERVICE_IID \
  {0x70bd93ff, 0x88fa, 0x4600, \
    { 0x8a, 0xf8, 0x57, 0xc8, 0xd0, 0x02, 0xdb, 0xac }}

class NS_NO_VTABLE nsICrashService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICRASHSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICrashService;

  /* Promise addCrash (in long processType, in long crashType, in AString id); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddCrash(int32_t processType, int32_t crashType, const nsAString& id, ::mozilla::dom::Promise * * _retval) = 0;

  enum {
    PROCESS_TYPE_MAIN = 0,
    PROCESS_TYPE_PLUGIN = 1,
    PROCESS_TYPE_CONTENT = 2,
    PROCESS_TYPE_IPDLUNITTEST = 3,
    PROCESS_TYPE_GMPLUGIN = 4,
    PROCESS_TYPE_GPU = 5,
    PROCESS_TYPE_VR = 6,
    PROCESS_TYPE_RDD = 7,
    PROCESS_TYPE_SOCKET = 8,
    PROCESS_TYPE_SANDBOX_BROKER = 9,
    PROCESS_TYPE_FORKSERVER = 10,
    CRASH_TYPE_CRASH = 0,
    CRASH_TYPE_HANG = 1
  };

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICrashService, NS_ICRASHSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICRASHSERVICE \
  NS_IMETHOD AddCrash(int32_t processType, int32_t crashType, const nsAString& id, ::mozilla::dom::Promise * * _retval) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICRASHSERVICE \
  nsresult AddCrash(int32_t processType, int32_t crashType, const nsAString& id, ::mozilla::dom::Promise * * _retval); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICRASHSERVICE(_to) \
  NS_IMETHOD AddCrash(int32_t processType, int32_t crashType, const nsAString& id, ::mozilla::dom::Promise * * _retval) override { return _to AddCrash(processType, crashType, id, _retval); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICRASHSERVICE(_to) \
  NS_IMETHOD AddCrash(int32_t processType, int32_t crashType, const nsAString& id, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddCrash(processType, crashType, id, _retval); } \


#endif /* __gen_nsICrashService_h__ */
