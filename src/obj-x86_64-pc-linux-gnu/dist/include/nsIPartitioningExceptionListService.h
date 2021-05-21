/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/antitracking/nsIPartitioningExceptionListService.idl
 */

#ifndef __gen_nsIPartitioningExceptionListService_h__
#define __gen_nsIPartitioningExceptionListService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIPartitioningExceptionListObserver */
#define NS_IPARTITIONINGEXCEPTIONLISTOBSERVER_IID_STR "d8db1086-7b59-44d3-9f88-f31a7e642637"

#define NS_IPARTITIONINGEXCEPTIONLISTOBSERVER_IID \
  {0xd8db1086, 0x7b59, 0x44d3, \
    { 0x9f, 0x88, 0xf3, 0x1a, 0x7e, 0x64, 0x26, 0x37 }}

class NS_NO_VTABLE nsIPartitioningExceptionListObserver : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPARTITIONINGEXCEPTIONLISTOBSERVER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPartitioningExceptionListObserver;

  /* void onExceptionListUpdate (in ACString aList); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnExceptionListUpdate(const nsACString& aList) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPartitioningExceptionListObserver, NS_IPARTITIONINGEXCEPTIONLISTOBSERVER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPARTITIONINGEXCEPTIONLISTOBSERVER \
  NS_IMETHOD OnExceptionListUpdate(const nsACString& aList) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPARTITIONINGEXCEPTIONLISTOBSERVER \
  nsresult OnExceptionListUpdate(const nsACString& aList); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPARTITIONINGEXCEPTIONLISTOBSERVER(_to) \
  NS_IMETHOD OnExceptionListUpdate(const nsACString& aList) override { return _to OnExceptionListUpdate(aList); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPARTITIONINGEXCEPTIONLISTOBSERVER(_to) \
  NS_IMETHOD OnExceptionListUpdate(const nsACString& aList) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnExceptionListUpdate(aList); } 


/* starting interface:    nsIPartitioningExceptionListService */
#define NS_IPARTITIONINGEXCEPTIONLISTSERVICE_IID_STR "cf83a9af-dd3f-43a2-88bb-489a22bca124"

#define NS_IPARTITIONINGEXCEPTIONLISTSERVICE_IID \
  {0xcf83a9af, 0xdd3f, 0x43a2, \
    { 0x88, 0xbb, 0x48, 0x9a, 0x22, 0xbc, 0xa1, 0x24 }}

class NS_NO_VTABLE nsIPartitioningExceptionListService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPARTITIONINGEXCEPTIONLISTSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPartitioningExceptionListService;

  /* void registerAndRunExceptionListObserver (in nsIPartitioningExceptionListObserver aObserver); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RegisterAndRunExceptionListObserver(nsIPartitioningExceptionListObserver *aObserver) = 0;

  /* void unregisterExceptionListObserver (in nsIPartitioningExceptionListObserver aObserver); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UnregisterExceptionListObserver(nsIPartitioningExceptionListObserver *aObserver) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPartitioningExceptionListService, NS_IPARTITIONINGEXCEPTIONLISTSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPARTITIONINGEXCEPTIONLISTSERVICE \
  NS_IMETHOD RegisterAndRunExceptionListObserver(nsIPartitioningExceptionListObserver *aObserver) override; \
  NS_IMETHOD UnregisterExceptionListObserver(nsIPartitioningExceptionListObserver *aObserver) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPARTITIONINGEXCEPTIONLISTSERVICE \
  nsresult RegisterAndRunExceptionListObserver(nsIPartitioningExceptionListObserver *aObserver); \
  nsresult UnregisterExceptionListObserver(nsIPartitioningExceptionListObserver *aObserver); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPARTITIONINGEXCEPTIONLISTSERVICE(_to) \
  NS_IMETHOD RegisterAndRunExceptionListObserver(nsIPartitioningExceptionListObserver *aObserver) override { return _to RegisterAndRunExceptionListObserver(aObserver); } \
  NS_IMETHOD UnregisterExceptionListObserver(nsIPartitioningExceptionListObserver *aObserver) override { return _to UnregisterExceptionListObserver(aObserver); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPARTITIONINGEXCEPTIONLISTSERVICE(_to) \
  NS_IMETHOD RegisterAndRunExceptionListObserver(nsIPartitioningExceptionListObserver *aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterAndRunExceptionListObserver(aObserver); } \
  NS_IMETHOD UnregisterExceptionListObserver(nsIPartitioningExceptionListObserver *aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnregisterExceptionListObserver(aObserver); } 


#endif /* __gen_nsIPartitioningExceptionListService_h__ */
