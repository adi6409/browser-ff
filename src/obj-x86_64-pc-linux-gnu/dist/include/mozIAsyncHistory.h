/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/places/mozIAsyncHistory.idl
 */

#ifndef __gen_mozIAsyncHistory_h__
#define __gen_mozIAsyncHistory_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class nsIVariant; /* forward declaration */


/* starting interface:    mozIVisitInfo */
#define MOZIVISITINFO_IID_STR "41e4ccc9-f0c8-4cd7-9753-7a38514b8488"

#define MOZIVISITINFO_IID \
  {0x41e4ccc9, 0xf0c8, 0x4cd7, \
    { 0x97, 0x53, 0x7a, 0x38, 0x51, 0x4b, 0x84, 0x88 }}

class NS_NO_VTABLE mozIVisitInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZIVISITINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIVisitInfo;

  /* readonly attribute long long visitId; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetVisitId(int64_t *aVisitId) = 0;

  /* readonly attribute PRTime visitDate; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetVisitDate(PRTime *aVisitDate) = 0;

  /* readonly attribute unsigned long transitionType; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTransitionType(uint32_t *aTransitionType) = 0;

  /* readonly attribute nsIURI referrerURI; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetReferrerURI(nsIURI **aReferrerURI) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIVisitInfo, MOZIVISITINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZIVISITINFO \
  NS_IMETHOD GetVisitId(int64_t *aVisitId) override; \
  NS_IMETHOD GetVisitDate(PRTime *aVisitDate) override; \
  NS_IMETHOD GetTransitionType(uint32_t *aTransitionType) override; \
  NS_IMETHOD GetReferrerURI(nsIURI **aReferrerURI) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZIVISITINFO \
  nsresult GetVisitId(int64_t *aVisitId); \
  nsresult GetVisitDate(PRTime *aVisitDate); \
  nsresult GetTransitionType(uint32_t *aTransitionType); \
  nsresult GetReferrerURI(nsIURI **aReferrerURI); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZIVISITINFO(_to) \
  NS_IMETHOD GetVisitId(int64_t *aVisitId) override { return _to GetVisitId(aVisitId); } \
  NS_IMETHOD GetVisitDate(PRTime *aVisitDate) override { return _to GetVisitDate(aVisitDate); } \
  NS_IMETHOD GetTransitionType(uint32_t *aTransitionType) override { return _to GetTransitionType(aTransitionType); } \
  NS_IMETHOD GetReferrerURI(nsIURI **aReferrerURI) override { return _to GetReferrerURI(aReferrerURI); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZIVISITINFO(_to) \
  NS_IMETHOD GetVisitId(int64_t *aVisitId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetVisitId(aVisitId); } \
  NS_IMETHOD GetVisitDate(PRTime *aVisitDate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetVisitDate(aVisitDate); } \
  NS_IMETHOD GetTransitionType(uint32_t *aTransitionType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTransitionType(aTransitionType); } \
  NS_IMETHOD GetReferrerURI(nsIURI **aReferrerURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetReferrerURI(aReferrerURI); } 


/* starting interface:    mozIPlaceInfo */
#define MOZIPLACEINFO_IID_STR "ad83e137-c92a-4b7b-b67e-0a318811f91e"

#define MOZIPLACEINFO_IID \
  {0xad83e137, 0xc92a, 0x4b7b, \
    { 0xb6, 0x7e, 0x0a, 0x31, 0x88, 0x11, 0xf9, 0x1e }}

class NS_NO_VTABLE mozIPlaceInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZIPLACEINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIPlaceInfo;

  /* readonly attribute long long placeId; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPlaceId(int64_t *aPlaceId) = 0;

  /* readonly attribute ACString guid; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetGuid(nsACString& aGuid) = 0;

  /* readonly attribute nsIURI uri; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUri(nsIURI **aUri) = 0;

  /* readonly attribute AString title; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTitle(nsAString& aTitle) = 0;

  /* readonly attribute long long frecency; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFrecency(int64_t *aFrecency) = 0;

  /* [implicit_jscontext] readonly attribute jsval visits; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetVisits(JSContext* cx, JS::MutableHandleValue aVisits) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIPlaceInfo, MOZIPLACEINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZIPLACEINFO \
  NS_IMETHOD GetPlaceId(int64_t *aPlaceId) override; \
  NS_IMETHOD GetGuid(nsACString& aGuid) override; \
  NS_IMETHOD GetUri(nsIURI **aUri) override; \
  NS_IMETHOD GetTitle(nsAString& aTitle) override; \
  NS_IMETHOD GetFrecency(int64_t *aFrecency) override; \
  NS_IMETHOD GetVisits(JSContext* cx, JS::MutableHandleValue aVisits) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZIPLACEINFO \
  nsresult GetPlaceId(int64_t *aPlaceId); \
  nsresult GetGuid(nsACString& aGuid); \
  nsresult GetUri(nsIURI **aUri); \
  nsresult GetTitle(nsAString& aTitle); \
  nsresult GetFrecency(int64_t *aFrecency); \
  nsresult GetVisits(JSContext* cx, JS::MutableHandleValue aVisits); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZIPLACEINFO(_to) \
  NS_IMETHOD GetPlaceId(int64_t *aPlaceId) override { return _to GetPlaceId(aPlaceId); } \
  NS_IMETHOD GetGuid(nsACString& aGuid) override { return _to GetGuid(aGuid); } \
  NS_IMETHOD GetUri(nsIURI **aUri) override { return _to GetUri(aUri); } \
  NS_IMETHOD GetTitle(nsAString& aTitle) override { return _to GetTitle(aTitle); } \
  NS_IMETHOD GetFrecency(int64_t *aFrecency) override { return _to GetFrecency(aFrecency); } \
  NS_IMETHOD GetVisits(JSContext* cx, JS::MutableHandleValue aVisits) override { return _to GetVisits(cx, aVisits); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZIPLACEINFO(_to) \
  NS_IMETHOD GetPlaceId(int64_t *aPlaceId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPlaceId(aPlaceId); } \
  NS_IMETHOD GetGuid(nsACString& aGuid) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetGuid(aGuid); } \
  NS_IMETHOD GetUri(nsIURI **aUri) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUri(aUri); } \
  NS_IMETHOD GetTitle(nsAString& aTitle) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTitle(aTitle); } \
  NS_IMETHOD GetFrecency(int64_t *aFrecency) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFrecency(aFrecency); } \
  NS_IMETHOD GetVisits(JSContext* cx, JS::MutableHandleValue aVisits) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetVisits(cx, aVisits); } 


/* starting interface:    mozIVisitInfoCallback */
#define MOZIVISITINFOCALLBACK_IID_STR "1f266877-2859-418b-a11b-ec3ae4f4f93d"

#define MOZIVISITINFOCALLBACK_IID \
  {0x1f266877, 0x2859, 0x418b, \
    { 0xa1, 0x1b, 0xec, 0x3a, 0xe4, 0xf4, 0xf9, 0x3d }}

class NS_NO_VTABLE mozIVisitInfoCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZIVISITINFOCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIVisitInfoCallback;

  /* void handleError (in nsresult aResultCode, in mozIPlaceInfo aPlaceInfo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleError(nsresult aResultCode, mozIPlaceInfo *aPlaceInfo) = 0;

  /* void handleResult (in mozIPlaceInfo aPlaceInfo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleResult(mozIPlaceInfo *aPlaceInfo) = 0;

  /* void handleCompletion (in unsigned long aUpdatedItems); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleCompletion(uint32_t aUpdatedItems) = 0;

  /* readonly attribute bool ignoreResults; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIgnoreResults(bool *aIgnoreResults) = 0;

  /* readonly attribute bool ignoreErrors; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIgnoreErrors(bool *aIgnoreErrors) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIVisitInfoCallback, MOZIVISITINFOCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZIVISITINFOCALLBACK \
  NS_IMETHOD HandleError(nsresult aResultCode, mozIPlaceInfo *aPlaceInfo) override; \
  NS_IMETHOD HandleResult(mozIPlaceInfo *aPlaceInfo) override; \
  NS_IMETHOD HandleCompletion(uint32_t aUpdatedItems) override; \
  NS_IMETHOD GetIgnoreResults(bool *aIgnoreResults) override; \
  NS_IMETHOD GetIgnoreErrors(bool *aIgnoreErrors) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZIVISITINFOCALLBACK \
  nsresult HandleError(nsresult aResultCode, mozIPlaceInfo *aPlaceInfo); \
  nsresult HandleResult(mozIPlaceInfo *aPlaceInfo); \
  nsresult HandleCompletion(uint32_t aUpdatedItems); \
  nsresult GetIgnoreResults(bool *aIgnoreResults); \
  nsresult GetIgnoreErrors(bool *aIgnoreErrors); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZIVISITINFOCALLBACK(_to) \
  NS_IMETHOD HandleError(nsresult aResultCode, mozIPlaceInfo *aPlaceInfo) override { return _to HandleError(aResultCode, aPlaceInfo); } \
  NS_IMETHOD HandleResult(mozIPlaceInfo *aPlaceInfo) override { return _to HandleResult(aPlaceInfo); } \
  NS_IMETHOD HandleCompletion(uint32_t aUpdatedItems) override { return _to HandleCompletion(aUpdatedItems); } \
  NS_IMETHOD GetIgnoreResults(bool *aIgnoreResults) override { return _to GetIgnoreResults(aIgnoreResults); } \
  NS_IMETHOD GetIgnoreErrors(bool *aIgnoreErrors) override { return _to GetIgnoreErrors(aIgnoreErrors); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZIVISITINFOCALLBACK(_to) \
  NS_IMETHOD HandleError(nsresult aResultCode, mozIPlaceInfo *aPlaceInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleError(aResultCode, aPlaceInfo); } \
  NS_IMETHOD HandleResult(mozIPlaceInfo *aPlaceInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleResult(aPlaceInfo); } \
  NS_IMETHOD HandleCompletion(uint32_t aUpdatedItems) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleCompletion(aUpdatedItems); } \
  NS_IMETHOD GetIgnoreResults(bool *aIgnoreResults) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIgnoreResults(aIgnoreResults); } \
  NS_IMETHOD GetIgnoreErrors(bool *aIgnoreErrors) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIgnoreErrors(aIgnoreErrors); } 


/* starting interface:    mozIVisitedStatusCallback */
#define MOZIVISITEDSTATUSCALLBACK_IID_STR "994092bf-936f-449b-8dd6-0941e024360d"

#define MOZIVISITEDSTATUSCALLBACK_IID \
  {0x994092bf, 0x936f, 0x449b, \
    { 0x8d, 0xd6, 0x09, 0x41, 0xe0, 0x24, 0x36, 0x0d }}

class NS_NO_VTABLE mozIVisitedStatusCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZIVISITEDSTATUSCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIVisitedStatusCallback;

  /* void isVisited (in nsIURI aURI, in boolean aVisitedStatus); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsVisited(nsIURI *aURI, bool aVisitedStatus) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIVisitedStatusCallback, MOZIVISITEDSTATUSCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZIVISITEDSTATUSCALLBACK \
  NS_IMETHOD IsVisited(nsIURI *aURI, bool aVisitedStatus) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZIVISITEDSTATUSCALLBACK \
  nsresult IsVisited(nsIURI *aURI, bool aVisitedStatus); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZIVISITEDSTATUSCALLBACK(_to) \
  NS_IMETHOD IsVisited(nsIURI *aURI, bool aVisitedStatus) override { return _to IsVisited(aURI, aVisitedStatus); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZIVISITEDSTATUSCALLBACK(_to) \
  NS_IMETHOD IsVisited(nsIURI *aURI, bool aVisitedStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsVisited(aURI, aVisitedStatus); } 


/* starting interface:    mozIAsyncHistory */
#define MOZIASYNCHISTORY_IID_STR "1643efd2-a329-4733-a39d-17069c8d3b2d"

#define MOZIASYNCHISTORY_IID \
  {0x1643efd2, 0xa329, 0x4733, \
    { 0xa3, 0x9d, 0x17, 0x06, 0x9c, 0x8d, 0x3b, 0x2d }}

class NS_NO_VTABLE mozIAsyncHistory : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZIASYNCHISTORY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIAsyncHistory;

  /* [implicit_jscontext] void updatePlaces (in jsval aPlaceInfo, [optional] in mozIVisitInfoCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UpdatePlaces(JS::HandleValue aPlaceInfo, mozIVisitInfoCallback *aCallback, JSContext* cx) = 0;

  /* void isURIVisited (in nsIURI aURI, in mozIVisitedStatusCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsURIVisited(nsIURI *aURI, mozIVisitedStatusCallback *aCallback) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIAsyncHistory, MOZIASYNCHISTORY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZIASYNCHISTORY \
  NS_IMETHOD UpdatePlaces(JS::HandleValue aPlaceInfo, mozIVisitInfoCallback *aCallback, JSContext* cx) override; \
  NS_IMETHOD IsURIVisited(nsIURI *aURI, mozIVisitedStatusCallback *aCallback) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZIASYNCHISTORY \
  nsresult UpdatePlaces(JS::HandleValue aPlaceInfo, mozIVisitInfoCallback *aCallback, JSContext* cx); \
  nsresult IsURIVisited(nsIURI *aURI, mozIVisitedStatusCallback *aCallback); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZIASYNCHISTORY(_to) \
  NS_IMETHOD UpdatePlaces(JS::HandleValue aPlaceInfo, mozIVisitInfoCallback *aCallback, JSContext* cx) override { return _to UpdatePlaces(aPlaceInfo, aCallback, cx); } \
  NS_IMETHOD IsURIVisited(nsIURI *aURI, mozIVisitedStatusCallback *aCallback) override { return _to IsURIVisited(aURI, aCallback); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZIASYNCHISTORY(_to) \
  NS_IMETHOD UpdatePlaces(JS::HandleValue aPlaceInfo, mozIVisitInfoCallback *aCallback, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UpdatePlaces(aPlaceInfo, aCallback, cx); } \
  NS_IMETHOD IsURIVisited(nsIURI *aURI, mozIVisitedStatusCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsURIVisited(aURI, aCallback); } 


#endif /* __gen_mozIAsyncHistory_h__ */
