/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/url-classifier/nsIUrlClassifierDBService.idl
 */

#ifndef __gen_nsIUrlClassifierDBService_h__
#define __gen_nsIUrlClassifierDBService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#include "Entries.h"
#include "LookupCache.h"
#include "mozilla/UniquePtr.h"
class nsIUrlClassifierHashCompleter; /* forward declaration */

class nsIPrincipal; /* forward declaration */


/* starting interface:    nsIUrlClassifierCallback */
#define NS_IURLCLASSIFIERCALLBACK_IID_STR "4ca27b6b-a674-4b3d-ab30-d21e2da2dffb"

#define NS_IURLCLASSIFIERCALLBACK_IID \
  {0x4ca27b6b, 0xa674, 0x4b3d, \
    { 0xab, 0x30, 0xd2, 0x1e, 0x2d, 0xa2, 0xdf, 0xfb }}

class NS_NO_VTABLE nsIUrlClassifierCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURLCLASSIFIERCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUrlClassifierCallback;

  /* void handleEvent (in ACString value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleEvent(const nsACString& value) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUrlClassifierCallback, NS_IURLCLASSIFIERCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURLCLASSIFIERCALLBACK \
  NS_IMETHOD HandleEvent(const nsACString& value) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURLCLASSIFIERCALLBACK \
  nsresult HandleEvent(const nsACString& value); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURLCLASSIFIERCALLBACK(_to) \
  NS_IMETHOD HandleEvent(const nsACString& value) override { return _to HandleEvent(value); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURLCLASSIFIERCALLBACK(_to) \
  NS_IMETHOD HandleEvent(const nsACString& value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleEvent(value); } 


/* starting interface:    nsIUrlClassifierUpdateObserver */
#define NS_IURLCLASSIFIERUPDATEOBSERVER_IID_STR "9fa11561-5816-4e1b-bcc9-b629ca05cce6"

#define NS_IURLCLASSIFIERUPDATEOBSERVER_IID \
  {0x9fa11561, 0x5816, 0x4e1b, \
    { 0xbc, 0xc9, 0xb6, 0x29, 0xca, 0x05, 0xcc, 0xe6 }}

class NS_NO_VTABLE nsIUrlClassifierUpdateObserver : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURLCLASSIFIERUPDATEOBSERVER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUrlClassifierUpdateObserver;

  /* void updateUrlRequested (in ACString url, in ACString table); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UpdateUrlRequested(const nsACString& url, const nsACString& table) = 0;

  /* void streamFinished (in nsresult status, in unsigned long delay); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD StreamFinished(nsresult status, uint32_t delay) = 0;

  /* void updateError (in nsresult error); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UpdateError(nsresult error) = 0;

  /* void updateSuccess (in unsigned long requestedTimeout); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UpdateSuccess(uint32_t requestedTimeout) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUrlClassifierUpdateObserver, NS_IURLCLASSIFIERUPDATEOBSERVER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURLCLASSIFIERUPDATEOBSERVER \
  NS_IMETHOD UpdateUrlRequested(const nsACString& url, const nsACString& table) override; \
  NS_IMETHOD StreamFinished(nsresult status, uint32_t delay) override; \
  NS_IMETHOD UpdateError(nsresult error) override; \
  NS_IMETHOD UpdateSuccess(uint32_t requestedTimeout) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURLCLASSIFIERUPDATEOBSERVER \
  nsresult UpdateUrlRequested(const nsACString& url, const nsACString& table); \
  nsresult StreamFinished(nsresult status, uint32_t delay); \
  nsresult UpdateError(nsresult error); \
  nsresult UpdateSuccess(uint32_t requestedTimeout); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURLCLASSIFIERUPDATEOBSERVER(_to) \
  NS_IMETHOD UpdateUrlRequested(const nsACString& url, const nsACString& table) override { return _to UpdateUrlRequested(url, table); } \
  NS_IMETHOD StreamFinished(nsresult status, uint32_t delay) override { return _to StreamFinished(status, delay); } \
  NS_IMETHOD UpdateError(nsresult error) override { return _to UpdateError(error); } \
  NS_IMETHOD UpdateSuccess(uint32_t requestedTimeout) override { return _to UpdateSuccess(requestedTimeout); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURLCLASSIFIERUPDATEOBSERVER(_to) \
  NS_IMETHOD UpdateUrlRequested(const nsACString& url, const nsACString& table) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UpdateUrlRequested(url, table); } \
  NS_IMETHOD StreamFinished(nsresult status, uint32_t delay) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StreamFinished(status, delay); } \
  NS_IMETHOD UpdateError(nsresult error) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UpdateError(error); } \
  NS_IMETHOD UpdateSuccess(uint32_t requestedTimeout) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UpdateSuccess(requestedTimeout); } 


/* starting interface:    nsIUrlClassifierDBService */
#define NS_IURLCLASSIFIERDBSERVICE_IID_STR "7a258022-6765-11e5-b379-b37b1f2354be"

#define NS_IURLCLASSIFIERDBSERVICE_IID \
  {0x7a258022, 0x6765, 0x11e5, \
    { 0xb3, 0x79, 0xb3, 0x7b, 0x1f, 0x23, 0x54, 0xbe }}

class NS_NO_VTABLE nsIUrlClassifierDBService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURLCLASSIFIERDBSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUrlClassifierDBService;

  /* void lookup (in nsIPrincipal principal, in ACString tables, in nsIUrlClassifierCallback c); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Lookup(nsIPrincipal *principal, const nsACString& tables, nsIUrlClassifierCallback *c) = 0;

  /* void getTables (in nsIUrlClassifierCallback c); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTables(nsIUrlClassifierCallback *c) = 0;

  /* void setHashCompleter (in ACString tableName, in nsIUrlClassifierHashCompleter completer); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetHashCompleter(const nsACString& tableName, nsIUrlClassifierHashCompleter *completer) = 0;

  /* void clearLastResults (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ClearLastResults(void) = 0;

  /* void beginUpdate (in nsIUrlClassifierUpdateObserver updater, in ACString tables); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD BeginUpdate(nsIUrlClassifierUpdateObserver *updater, const nsACString& tables) = 0;

  /* void beginStream (in ACString table); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD BeginStream(const nsACString& table) = 0;

  /* void updateStream (in ACString updateChunk); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UpdateStream(const nsACString& updateChunk) = 0;

  /* void finishStream (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FinishStream(void) = 0;

  /* void finishUpdate (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FinishUpdate(void) = 0;

  /* void cancelUpdate (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CancelUpdate(void) = 0;

  /* void resetDatabase (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ResetDatabase(void) = 0;

  /* void reloadDatabase (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ReloadDatabase(void) = 0;

  /* void clearCache (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ClearCache(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUrlClassifierDBService, NS_IURLCLASSIFIERDBSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURLCLASSIFIERDBSERVICE \
  NS_IMETHOD Lookup(nsIPrincipal *principal, const nsACString& tables, nsIUrlClassifierCallback *c) override; \
  NS_IMETHOD GetTables(nsIUrlClassifierCallback *c) override; \
  NS_IMETHOD SetHashCompleter(const nsACString& tableName, nsIUrlClassifierHashCompleter *completer) override; \
  NS_IMETHOD ClearLastResults(void) override; \
  NS_IMETHOD BeginUpdate(nsIUrlClassifierUpdateObserver *updater, const nsACString& tables) override; \
  NS_IMETHOD BeginStream(const nsACString& table) override; \
  NS_IMETHOD UpdateStream(const nsACString& updateChunk) override; \
  NS_IMETHOD FinishStream(void) override; \
  NS_IMETHOD FinishUpdate(void) override; \
  NS_IMETHOD CancelUpdate(void) override; \
  NS_IMETHOD ResetDatabase(void) override; \
  NS_IMETHOD ReloadDatabase(void) override; \
  NS_IMETHOD ClearCache(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURLCLASSIFIERDBSERVICE \
  nsresult Lookup(nsIPrincipal *principal, const nsACString& tables, nsIUrlClassifierCallback *c); \
  nsresult GetTables(nsIUrlClassifierCallback *c); \
  nsresult SetHashCompleter(const nsACString& tableName, nsIUrlClassifierHashCompleter *completer); \
  nsresult ClearLastResults(void); \
  nsresult BeginUpdate(nsIUrlClassifierUpdateObserver *updater, const nsACString& tables); \
  nsresult BeginStream(const nsACString& table); \
  nsresult UpdateStream(const nsACString& updateChunk); \
  nsresult FinishStream(void); \
  nsresult FinishUpdate(void); \
  nsresult CancelUpdate(void); \
  nsresult ResetDatabase(void); \
  nsresult ReloadDatabase(void); \
  nsresult ClearCache(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURLCLASSIFIERDBSERVICE(_to) \
  NS_IMETHOD Lookup(nsIPrincipal *principal, const nsACString& tables, nsIUrlClassifierCallback *c) override { return _to Lookup(principal, tables, c); } \
  NS_IMETHOD GetTables(nsIUrlClassifierCallback *c) override { return _to GetTables(c); } \
  NS_IMETHOD SetHashCompleter(const nsACString& tableName, nsIUrlClassifierHashCompleter *completer) override { return _to SetHashCompleter(tableName, completer); } \
  NS_IMETHOD ClearLastResults(void) override { return _to ClearLastResults(); } \
  NS_IMETHOD BeginUpdate(nsIUrlClassifierUpdateObserver *updater, const nsACString& tables) override { return _to BeginUpdate(updater, tables); } \
  NS_IMETHOD BeginStream(const nsACString& table) override { return _to BeginStream(table); } \
  NS_IMETHOD UpdateStream(const nsACString& updateChunk) override { return _to UpdateStream(updateChunk); } \
  NS_IMETHOD FinishStream(void) override { return _to FinishStream(); } \
  NS_IMETHOD FinishUpdate(void) override { return _to FinishUpdate(); } \
  NS_IMETHOD CancelUpdate(void) override { return _to CancelUpdate(); } \
  NS_IMETHOD ResetDatabase(void) override { return _to ResetDatabase(); } \
  NS_IMETHOD ReloadDatabase(void) override { return _to ReloadDatabase(); } \
  NS_IMETHOD ClearCache(void) override { return _to ClearCache(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURLCLASSIFIERDBSERVICE(_to) \
  NS_IMETHOD Lookup(nsIPrincipal *principal, const nsACString& tables, nsIUrlClassifierCallback *c) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Lookup(principal, tables, c); } \
  NS_IMETHOD GetTables(nsIUrlClassifierCallback *c) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTables(c); } \
  NS_IMETHOD SetHashCompleter(const nsACString& tableName, nsIUrlClassifierHashCompleter *completer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetHashCompleter(tableName, completer); } \
  NS_IMETHOD ClearLastResults(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearLastResults(); } \
  NS_IMETHOD BeginUpdate(nsIUrlClassifierUpdateObserver *updater, const nsACString& tables) override { return !_to ? NS_ERROR_NULL_POINTER : _to->BeginUpdate(updater, tables); } \
  NS_IMETHOD BeginStream(const nsACString& table) override { return !_to ? NS_ERROR_NULL_POINTER : _to->BeginStream(table); } \
  NS_IMETHOD UpdateStream(const nsACString& updateChunk) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UpdateStream(updateChunk); } \
  NS_IMETHOD FinishStream(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FinishStream(); } \
  NS_IMETHOD FinishUpdate(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FinishUpdate(); } \
  NS_IMETHOD CancelUpdate(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CancelUpdate(); } \
  NS_IMETHOD ResetDatabase(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResetDatabase(); } \
  NS_IMETHOD ReloadDatabase(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReloadDatabase(); } \
  NS_IMETHOD ClearCache(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearCache(); } 


/* starting interface:    nsIUrlClassifierLookupCallback */
#define NS_IURLCLASSIFIERLOOKUPCALLBACK_IID_STR "b903dc8f-dff1-42fe-894b-36e7a59bb801"

#define NS_IURLCLASSIFIERLOOKUPCALLBACK_IID \
  {0xb903dc8f, 0xdff1, 0x42fe, \
    { 0x89, 0x4b, 0x36, 0xe7, 0xa5, 0x9b, 0xb8, 0x01 }}

class NS_NO_VTABLE nsIUrlClassifierLookupCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURLCLASSIFIERLOOKUPCALLBACK_IID)

  /* void lookupComplete (in ResultArray results); */
  NS_IMETHOD LookupComplete(mozilla::UniquePtr<mozilla::safebrowsing::LookupResultArray> results) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUrlClassifierLookupCallback, NS_IURLCLASSIFIERLOOKUPCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURLCLASSIFIERLOOKUPCALLBACK \
  NS_IMETHOD LookupComplete(mozilla::UniquePtr<mozilla::safebrowsing::LookupResultArray> results) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURLCLASSIFIERLOOKUPCALLBACK \
  nsresult LookupComplete(mozilla::UniquePtr<mozilla::safebrowsing::LookupResultArray> results); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURLCLASSIFIERLOOKUPCALLBACK(_to) \
  NS_IMETHOD LookupComplete(mozilla::UniquePtr<mozilla::safebrowsing::LookupResultArray> results) override { return _to LookupComplete(results); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURLCLASSIFIERLOOKUPCALLBACK(_to) \
  NS_IMETHOD LookupComplete(mozilla::UniquePtr<mozilla::safebrowsing::LookupResultArray> results) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LookupComplete(results); } 


/* starting interface:    nsIUrlClassifierClassifyCallback */
#define NS_IURLCLASSIFIERCLASSIFYCALLBACK_IID_STR "091adf98-28a5-473d-8dec-5b34b4e62496"

#define NS_IURLCLASSIFIERCLASSIFYCALLBACK_IID \
  {0x091adf98, 0x28a5, 0x473d, \
    { 0x8d, 0xec, 0x5b, 0x34, 0xb4, 0xe6, 0x24, 0x96 }}

class NS_NO_VTABLE nsIUrlClassifierClassifyCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURLCLASSIFIERCLASSIFYCALLBACK_IID)

  /* void handleResult (in ACString aList, in ACString aPrefix); */
  NS_IMETHOD HandleResult(const nsACString& aList, const nsACString& aPrefix) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUrlClassifierClassifyCallback, NS_IURLCLASSIFIERCLASSIFYCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURLCLASSIFIERCLASSIFYCALLBACK \
  NS_IMETHOD HandleResult(const nsACString& aList, const nsACString& aPrefix) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURLCLASSIFIERCLASSIFYCALLBACK \
  nsresult HandleResult(const nsACString& aList, const nsACString& aPrefix); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURLCLASSIFIERCLASSIFYCALLBACK(_to) \
  NS_IMETHOD HandleResult(const nsACString& aList, const nsACString& aPrefix) override { return _to HandleResult(aList, aPrefix); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURLCLASSIFIERCLASSIFYCALLBACK(_to) \
  NS_IMETHOD HandleResult(const nsACString& aList, const nsACString& aPrefix) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleResult(aList, aPrefix); } 


#endif /* __gen_nsIUrlClassifierDBService_h__ */
