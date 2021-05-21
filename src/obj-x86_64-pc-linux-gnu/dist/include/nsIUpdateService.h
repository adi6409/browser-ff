/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/mozapps/update/nsIUpdateService.idl
 */

#ifndef __gen_nsIUpdateService_h__
#define __gen_nsIUpdateService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIRequest; /* forward declaration */

class nsIRequestObserver; /* forward declaration */

class nsISimpleEnumerator; /* forward declaration */

class nsIFile; /* forward declaration */

namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla

namespace mozilla {
namespace dom {
class Document; /* webidl Document */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIUpdatePatch */
#define NS_IUPDATEPATCH_IID_STR "dc8fb8a9-3a53-4031-9469-2a5197ea30e7"

#define NS_IUPDATEPATCH_IID \
  {0xdc8fb8a9, 0x3a53, 0x4031, \
    { 0x94, 0x69, 0x2a, 0x51, 0x97, 0xea, 0x30, 0xe7 }}

class NS_NO_VTABLE nsIUpdatePatch : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IUPDATEPATCH_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUpdatePatch;

  /* readonly attribute AString type; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetType(nsAString& aType) = 0;

  /* readonly attribute AString URL; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetURL(nsAString& aURL) = 0;

  /* attribute AString finalURL; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFinalURL(nsAString& aFinalURL) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetFinalURL(const nsAString& aFinalURL) = 0;

  /* readonly attribute unsigned long size; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSize(uint32_t *aSize) = 0;

  /* attribute AString state; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetState(nsAString& aState) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetState(const nsAString& aState) = 0;

  /* attribute long errorCode; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetErrorCode(int32_t *aErrorCode) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetErrorCode(int32_t aErrorCode) = 0;

  /* attribute boolean selected; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSelected(bool *aSelected) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSelected(bool aSelected) = 0;

  /* Element serialize (in Document updates); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Serialize(mozilla::dom::Document *updates, mozilla::dom::Element **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUpdatePatch, NS_IUPDATEPATCH_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIUPDATEPATCH \
  NS_IMETHOD GetType(nsAString& aType) override; \
  NS_IMETHOD GetURL(nsAString& aURL) override; \
  NS_IMETHOD GetFinalURL(nsAString& aFinalURL) override; \
  NS_IMETHOD SetFinalURL(const nsAString& aFinalURL) override; \
  NS_IMETHOD GetSize(uint32_t *aSize) override; \
  NS_IMETHOD GetState(nsAString& aState) override; \
  NS_IMETHOD SetState(const nsAString& aState) override; \
  NS_IMETHOD GetErrorCode(int32_t *aErrorCode) override; \
  NS_IMETHOD SetErrorCode(int32_t aErrorCode) override; \
  NS_IMETHOD GetSelected(bool *aSelected) override; \
  NS_IMETHOD SetSelected(bool aSelected) override; \
  NS_IMETHOD Serialize(mozilla::dom::Document *updates, mozilla::dom::Element **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIUPDATEPATCH \
  nsresult GetType(nsAString& aType); \
  nsresult GetURL(nsAString& aURL); \
  nsresult GetFinalURL(nsAString& aFinalURL); \
  nsresult SetFinalURL(const nsAString& aFinalURL); \
  nsresult GetSize(uint32_t *aSize); \
  nsresult GetState(nsAString& aState); \
  nsresult SetState(const nsAString& aState); \
  nsresult GetErrorCode(int32_t *aErrorCode); \
  nsresult SetErrorCode(int32_t aErrorCode); \
  nsresult GetSelected(bool *aSelected); \
  nsresult SetSelected(bool aSelected); \
  nsresult Serialize(mozilla::dom::Document *updates, mozilla::dom::Element **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIUPDATEPATCH(_to) \
  NS_IMETHOD GetType(nsAString& aType) override { return _to GetType(aType); } \
  NS_IMETHOD GetURL(nsAString& aURL) override { return _to GetURL(aURL); } \
  NS_IMETHOD GetFinalURL(nsAString& aFinalURL) override { return _to GetFinalURL(aFinalURL); } \
  NS_IMETHOD SetFinalURL(const nsAString& aFinalURL) override { return _to SetFinalURL(aFinalURL); } \
  NS_IMETHOD GetSize(uint32_t *aSize) override { return _to GetSize(aSize); } \
  NS_IMETHOD GetState(nsAString& aState) override { return _to GetState(aState); } \
  NS_IMETHOD SetState(const nsAString& aState) override { return _to SetState(aState); } \
  NS_IMETHOD GetErrorCode(int32_t *aErrorCode) override { return _to GetErrorCode(aErrorCode); } \
  NS_IMETHOD SetErrorCode(int32_t aErrorCode) override { return _to SetErrorCode(aErrorCode); } \
  NS_IMETHOD GetSelected(bool *aSelected) override { return _to GetSelected(aSelected); } \
  NS_IMETHOD SetSelected(bool aSelected) override { return _to SetSelected(aSelected); } \
  NS_IMETHOD Serialize(mozilla::dom::Document *updates, mozilla::dom::Element **_retval) override { return _to Serialize(updates, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIUPDATEPATCH(_to) \
  NS_IMETHOD GetType(nsAString& aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetType(aType); } \
  NS_IMETHOD GetURL(nsAString& aURL) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetURL(aURL); } \
  NS_IMETHOD GetFinalURL(nsAString& aFinalURL) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFinalURL(aFinalURL); } \
  NS_IMETHOD SetFinalURL(const nsAString& aFinalURL) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFinalURL(aFinalURL); } \
  NS_IMETHOD GetSize(uint32_t *aSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSize(aSize); } \
  NS_IMETHOD GetState(nsAString& aState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetState(aState); } \
  NS_IMETHOD SetState(const nsAString& aState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetState(aState); } \
  NS_IMETHOD GetErrorCode(int32_t *aErrorCode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetErrorCode(aErrorCode); } \
  NS_IMETHOD SetErrorCode(int32_t aErrorCode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetErrorCode(aErrorCode); } \
  NS_IMETHOD GetSelected(bool *aSelected) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelected(aSelected); } \
  NS_IMETHOD SetSelected(bool aSelected) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSelected(aSelected); } \
  NS_IMETHOD Serialize(mozilla::dom::Document *updates, mozilla::dom::Element **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Serialize(updates, _retval); } 


/* starting interface:    nsIUpdate */
#define NS_IUPDATE_IID_STR "e094c045-f4ff-41fd-92da-cd2effd2c7c9"

#define NS_IUPDATE_IID \
  {0xe094c045, 0xf4ff, 0x41fd, \
    { 0x92, 0xda, 0xcd, 0x2e, 0xff, 0xd2, 0xc7, 0xc9 }}

class NS_NO_VTABLE nsIUpdate : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IUPDATE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUpdate;

  /* readonly attribute AString type; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetType(nsAString& aType) = 0;

  /* readonly attribute AString name; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetName(nsAString& aName) = 0;

  /* readonly attribute AString displayVersion; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDisplayVersion(nsAString& aDisplayVersion) = 0;

  /* readonly attribute AString appVersion; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAppVersion(nsAString& aAppVersion) = 0;

  /* readonly attribute AString previousAppVersion; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPreviousAppVersion(nsAString& aPreviousAppVersion) = 0;

  /* readonly attribute AString buildID; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetBuildID(nsAString& aBuildID) = 0;

  /* readonly attribute AString detailsURL; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDetailsURL(nsAString& aDetailsURL) = 0;

  /* readonly attribute AString serviceURL; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetServiceURL(nsAString& aServiceURL) = 0;

  /* readonly attribute AString channel; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetChannel(nsAString& aChannel) = 0;

  /* readonly attribute boolean unsupported; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUnsupported(bool *aUnsupported) = 0;

  /* attribute long long promptWaitTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPromptWaitTime(int64_t *aPromptWaitTime) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPromptWaitTime(int64_t aPromptWaitTime) = 0;

  /* attribute boolean isCompleteUpdate; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsCompleteUpdate(bool *aIsCompleteUpdate) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetIsCompleteUpdate(bool aIsCompleteUpdate) = 0;

  /* attribute long long installDate; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetInstallDate(int64_t *aInstallDate) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetInstallDate(int64_t aInstallDate) = 0;

  /* attribute AString statusText; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetStatusText(nsAString& aStatusText) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetStatusText(const nsAString& aStatusText) = 0;

  /* readonly attribute nsIUpdatePatch selectedPatch; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSelectedPatch(nsIUpdatePatch **aSelectedPatch) = 0;

  /* attribute AString state; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetState(nsAString& aState) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetState(const nsAString& aState) = 0;

  /* attribute long errorCode; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetErrorCode(int32_t *aErrorCode) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetErrorCode(int32_t aErrorCode) = 0;

  /* attribute boolean elevationFailure; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetElevationFailure(bool *aElevationFailure) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetElevationFailure(bool aElevationFailure) = 0;

  /* readonly attribute unsigned long patchCount; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPatchCount(uint32_t *aPatchCount) = 0;

  /* nsIUpdatePatch getPatchAt (in unsigned long index); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPatchAt(uint32_t index, nsIUpdatePatch **_retval) = 0;

  /* Element serialize (in Document updates); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Serialize(mozilla::dom::Document *updates, mozilla::dom::Element **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUpdate, NS_IUPDATE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIUPDATE \
  NS_IMETHOD GetType(nsAString& aType) override; \
  NS_IMETHOD GetName(nsAString& aName) override; \
  NS_IMETHOD GetDisplayVersion(nsAString& aDisplayVersion) override; \
  NS_IMETHOD GetAppVersion(nsAString& aAppVersion) override; \
  NS_IMETHOD GetPreviousAppVersion(nsAString& aPreviousAppVersion) override; \
  NS_IMETHOD GetBuildID(nsAString& aBuildID) override; \
  NS_IMETHOD GetDetailsURL(nsAString& aDetailsURL) override; \
  NS_IMETHOD GetServiceURL(nsAString& aServiceURL) override; \
  NS_IMETHOD GetChannel(nsAString& aChannel) override; \
  NS_IMETHOD GetUnsupported(bool *aUnsupported) override; \
  NS_IMETHOD GetPromptWaitTime(int64_t *aPromptWaitTime) override; \
  NS_IMETHOD SetPromptWaitTime(int64_t aPromptWaitTime) override; \
  NS_IMETHOD GetIsCompleteUpdate(bool *aIsCompleteUpdate) override; \
  NS_IMETHOD SetIsCompleteUpdate(bool aIsCompleteUpdate) override; \
  NS_IMETHOD GetInstallDate(int64_t *aInstallDate) override; \
  NS_IMETHOD SetInstallDate(int64_t aInstallDate) override; \
  NS_IMETHOD GetStatusText(nsAString& aStatusText) override; \
  NS_IMETHOD SetStatusText(const nsAString& aStatusText) override; \
  NS_IMETHOD GetSelectedPatch(nsIUpdatePatch **aSelectedPatch) override; \
  NS_IMETHOD GetState(nsAString& aState) override; \
  NS_IMETHOD SetState(const nsAString& aState) override; \
  NS_IMETHOD GetErrorCode(int32_t *aErrorCode) override; \
  NS_IMETHOD SetErrorCode(int32_t aErrorCode) override; \
  NS_IMETHOD GetElevationFailure(bool *aElevationFailure) override; \
  NS_IMETHOD SetElevationFailure(bool aElevationFailure) override; \
  NS_IMETHOD GetPatchCount(uint32_t *aPatchCount) override; \
  NS_IMETHOD GetPatchAt(uint32_t index, nsIUpdatePatch **_retval) override; \
  NS_IMETHOD Serialize(mozilla::dom::Document *updates, mozilla::dom::Element **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIUPDATE \
  nsresult GetType(nsAString& aType); \
  nsresult GetName(nsAString& aName); \
  nsresult GetDisplayVersion(nsAString& aDisplayVersion); \
  nsresult GetAppVersion(nsAString& aAppVersion); \
  nsresult GetPreviousAppVersion(nsAString& aPreviousAppVersion); \
  nsresult GetBuildID(nsAString& aBuildID); \
  nsresult GetDetailsURL(nsAString& aDetailsURL); \
  nsresult GetServiceURL(nsAString& aServiceURL); \
  nsresult GetChannel(nsAString& aChannel); \
  nsresult GetUnsupported(bool *aUnsupported); \
  nsresult GetPromptWaitTime(int64_t *aPromptWaitTime); \
  nsresult SetPromptWaitTime(int64_t aPromptWaitTime); \
  nsresult GetIsCompleteUpdate(bool *aIsCompleteUpdate); \
  nsresult SetIsCompleteUpdate(bool aIsCompleteUpdate); \
  nsresult GetInstallDate(int64_t *aInstallDate); \
  nsresult SetInstallDate(int64_t aInstallDate); \
  nsresult GetStatusText(nsAString& aStatusText); \
  nsresult SetStatusText(const nsAString& aStatusText); \
  nsresult GetSelectedPatch(nsIUpdatePatch **aSelectedPatch); \
  nsresult GetState(nsAString& aState); \
  nsresult SetState(const nsAString& aState); \
  nsresult GetErrorCode(int32_t *aErrorCode); \
  nsresult SetErrorCode(int32_t aErrorCode); \
  nsresult GetElevationFailure(bool *aElevationFailure); \
  nsresult SetElevationFailure(bool aElevationFailure); \
  nsresult GetPatchCount(uint32_t *aPatchCount); \
  nsresult GetPatchAt(uint32_t index, nsIUpdatePatch **_retval); \
  nsresult Serialize(mozilla::dom::Document *updates, mozilla::dom::Element **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIUPDATE(_to) \
  NS_IMETHOD GetType(nsAString& aType) override { return _to GetType(aType); } \
  NS_IMETHOD GetName(nsAString& aName) override { return _to GetName(aName); } \
  NS_IMETHOD GetDisplayVersion(nsAString& aDisplayVersion) override { return _to GetDisplayVersion(aDisplayVersion); } \
  NS_IMETHOD GetAppVersion(nsAString& aAppVersion) override { return _to GetAppVersion(aAppVersion); } \
  NS_IMETHOD GetPreviousAppVersion(nsAString& aPreviousAppVersion) override { return _to GetPreviousAppVersion(aPreviousAppVersion); } \
  NS_IMETHOD GetBuildID(nsAString& aBuildID) override { return _to GetBuildID(aBuildID); } \
  NS_IMETHOD GetDetailsURL(nsAString& aDetailsURL) override { return _to GetDetailsURL(aDetailsURL); } \
  NS_IMETHOD GetServiceURL(nsAString& aServiceURL) override { return _to GetServiceURL(aServiceURL); } \
  NS_IMETHOD GetChannel(nsAString& aChannel) override { return _to GetChannel(aChannel); } \
  NS_IMETHOD GetUnsupported(bool *aUnsupported) override { return _to GetUnsupported(aUnsupported); } \
  NS_IMETHOD GetPromptWaitTime(int64_t *aPromptWaitTime) override { return _to GetPromptWaitTime(aPromptWaitTime); } \
  NS_IMETHOD SetPromptWaitTime(int64_t aPromptWaitTime) override { return _to SetPromptWaitTime(aPromptWaitTime); } \
  NS_IMETHOD GetIsCompleteUpdate(bool *aIsCompleteUpdate) override { return _to GetIsCompleteUpdate(aIsCompleteUpdate); } \
  NS_IMETHOD SetIsCompleteUpdate(bool aIsCompleteUpdate) override { return _to SetIsCompleteUpdate(aIsCompleteUpdate); } \
  NS_IMETHOD GetInstallDate(int64_t *aInstallDate) override { return _to GetInstallDate(aInstallDate); } \
  NS_IMETHOD SetInstallDate(int64_t aInstallDate) override { return _to SetInstallDate(aInstallDate); } \
  NS_IMETHOD GetStatusText(nsAString& aStatusText) override { return _to GetStatusText(aStatusText); } \
  NS_IMETHOD SetStatusText(const nsAString& aStatusText) override { return _to SetStatusText(aStatusText); } \
  NS_IMETHOD GetSelectedPatch(nsIUpdatePatch **aSelectedPatch) override { return _to GetSelectedPatch(aSelectedPatch); } \
  NS_IMETHOD GetState(nsAString& aState) override { return _to GetState(aState); } \
  NS_IMETHOD SetState(const nsAString& aState) override { return _to SetState(aState); } \
  NS_IMETHOD GetErrorCode(int32_t *aErrorCode) override { return _to GetErrorCode(aErrorCode); } \
  NS_IMETHOD SetErrorCode(int32_t aErrorCode) override { return _to SetErrorCode(aErrorCode); } \
  NS_IMETHOD GetElevationFailure(bool *aElevationFailure) override { return _to GetElevationFailure(aElevationFailure); } \
  NS_IMETHOD SetElevationFailure(bool aElevationFailure) override { return _to SetElevationFailure(aElevationFailure); } \
  NS_IMETHOD GetPatchCount(uint32_t *aPatchCount) override { return _to GetPatchCount(aPatchCount); } \
  NS_IMETHOD GetPatchAt(uint32_t index, nsIUpdatePatch **_retval) override { return _to GetPatchAt(index, _retval); } \
  NS_IMETHOD Serialize(mozilla::dom::Document *updates, mozilla::dom::Element **_retval) override { return _to Serialize(updates, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIUPDATE(_to) \
  NS_IMETHOD GetType(nsAString& aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetType(aType); } \
  NS_IMETHOD GetName(nsAString& aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetName(aName); } \
  NS_IMETHOD GetDisplayVersion(nsAString& aDisplayVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDisplayVersion(aDisplayVersion); } \
  NS_IMETHOD GetAppVersion(nsAString& aAppVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAppVersion(aAppVersion); } \
  NS_IMETHOD GetPreviousAppVersion(nsAString& aPreviousAppVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPreviousAppVersion(aPreviousAppVersion); } \
  NS_IMETHOD GetBuildID(nsAString& aBuildID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBuildID(aBuildID); } \
  NS_IMETHOD GetDetailsURL(nsAString& aDetailsURL) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDetailsURL(aDetailsURL); } \
  NS_IMETHOD GetServiceURL(nsAString& aServiceURL) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetServiceURL(aServiceURL); } \
  NS_IMETHOD GetChannel(nsAString& aChannel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChannel(aChannel); } \
  NS_IMETHOD GetUnsupported(bool *aUnsupported) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUnsupported(aUnsupported); } \
  NS_IMETHOD GetPromptWaitTime(int64_t *aPromptWaitTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPromptWaitTime(aPromptWaitTime); } \
  NS_IMETHOD SetPromptWaitTime(int64_t aPromptWaitTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPromptWaitTime(aPromptWaitTime); } \
  NS_IMETHOD GetIsCompleteUpdate(bool *aIsCompleteUpdate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsCompleteUpdate(aIsCompleteUpdate); } \
  NS_IMETHOD SetIsCompleteUpdate(bool aIsCompleteUpdate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetIsCompleteUpdate(aIsCompleteUpdate); } \
  NS_IMETHOD GetInstallDate(int64_t *aInstallDate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInstallDate(aInstallDate); } \
  NS_IMETHOD SetInstallDate(int64_t aInstallDate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetInstallDate(aInstallDate); } \
  NS_IMETHOD GetStatusText(nsAString& aStatusText) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStatusText(aStatusText); } \
  NS_IMETHOD SetStatusText(const nsAString& aStatusText) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetStatusText(aStatusText); } \
  NS_IMETHOD GetSelectedPatch(nsIUpdatePatch **aSelectedPatch) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelectedPatch(aSelectedPatch); } \
  NS_IMETHOD GetState(nsAString& aState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetState(aState); } \
  NS_IMETHOD SetState(const nsAString& aState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetState(aState); } \
  NS_IMETHOD GetErrorCode(int32_t *aErrorCode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetErrorCode(aErrorCode); } \
  NS_IMETHOD SetErrorCode(int32_t aErrorCode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetErrorCode(aErrorCode); } \
  NS_IMETHOD GetElevationFailure(bool *aElevationFailure) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetElevationFailure(aElevationFailure); } \
  NS_IMETHOD SetElevationFailure(bool aElevationFailure) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetElevationFailure(aElevationFailure); } \
  NS_IMETHOD GetPatchCount(uint32_t *aPatchCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPatchCount(aPatchCount); } \
  NS_IMETHOD GetPatchAt(uint32_t index, nsIUpdatePatch **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPatchAt(index, _retval); } \
  NS_IMETHOD Serialize(mozilla::dom::Document *updates, mozilla::dom::Element **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Serialize(updates, _retval); } 


/* starting interface:    nsIUpdateCheckListener */
#define NS_IUPDATECHECKLISTENER_IID_STR "4aa2b4bb-39ea-407b-98ff-89f19134d4c0"

#define NS_IUPDATECHECKLISTENER_IID \
  {0x4aa2b4bb, 0x39ea, 0x407b, \
    { 0x98, 0xff, 0x89, 0xf1, 0x91, 0x34, 0xd4, 0xc0 }}

class NS_NO_VTABLE nsIUpdateCheckListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IUPDATECHECKLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUpdateCheckListener;

  /* void onCheckComplete (in jsval request, in Array<nsIUpdate> updates); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnCheckComplete(JS::HandleValue request, const nsTArray<RefPtr<nsIUpdate>>& updates) = 0;

  /* void onError (in jsval request, in nsIUpdate update); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnError(JS::HandleValue request, nsIUpdate *update) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUpdateCheckListener, NS_IUPDATECHECKLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIUPDATECHECKLISTENER \
  NS_IMETHOD OnCheckComplete(JS::HandleValue request, const nsTArray<RefPtr<nsIUpdate>>& updates) override; \
  NS_IMETHOD OnError(JS::HandleValue request, nsIUpdate *update) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIUPDATECHECKLISTENER \
  nsresult OnCheckComplete(JS::HandleValue request, const nsTArray<RefPtr<nsIUpdate>>& updates); \
  nsresult OnError(JS::HandleValue request, nsIUpdate *update); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIUPDATECHECKLISTENER(_to) \
  NS_IMETHOD OnCheckComplete(JS::HandleValue request, const nsTArray<RefPtr<nsIUpdate>>& updates) override { return _to OnCheckComplete(request, updates); } \
  NS_IMETHOD OnError(JS::HandleValue request, nsIUpdate *update) override { return _to OnError(request, update); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIUPDATECHECKLISTENER(_to) \
  NS_IMETHOD OnCheckComplete(JS::HandleValue request, const nsTArray<RefPtr<nsIUpdate>>& updates) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnCheckComplete(request, updates); } \
  NS_IMETHOD OnError(JS::HandleValue request, nsIUpdate *update) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnError(request, update); } 


/* starting interface:    nsIUpdateChecker */
#define NS_IUPDATECHECKER_IID_STR "877ace25-8bc5-452a-8586-9c1cf2871994"

#define NS_IUPDATECHECKER_IID \
  {0x877ace25, 0x8bc5, 0x452a, \
    { 0x85, 0x86, 0x9c, 0x1c, 0xf2, 0x87, 0x19, 0x94 }}

class NS_NO_VTABLE nsIUpdateChecker : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IUPDATECHECKER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUpdateChecker;

  /* void checkForUpdates (in nsIUpdateCheckListener listener, in boolean force); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CheckForUpdates(nsIUpdateCheckListener *listener, bool force) = 0;

  /* void stopCurrentCheck (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD StopCurrentCheck(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUpdateChecker, NS_IUPDATECHECKER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIUPDATECHECKER \
  NS_IMETHOD CheckForUpdates(nsIUpdateCheckListener *listener, bool force) override; \
  NS_IMETHOD StopCurrentCheck(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIUPDATECHECKER \
  nsresult CheckForUpdates(nsIUpdateCheckListener *listener, bool force); \
  nsresult StopCurrentCheck(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIUPDATECHECKER(_to) \
  NS_IMETHOD CheckForUpdates(nsIUpdateCheckListener *listener, bool force) override { return _to CheckForUpdates(listener, force); } \
  NS_IMETHOD StopCurrentCheck(void) override { return _to StopCurrentCheck(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIUPDATECHECKER(_to) \
  NS_IMETHOD CheckForUpdates(nsIUpdateCheckListener *listener, bool force) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CheckForUpdates(listener, force); } \
  NS_IMETHOD StopCurrentCheck(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StopCurrentCheck(); } 


/* starting interface:    nsIApplicationUpdateService */
#define NS_IAPPLICATIONUPDATESERVICE_IID_STR "1107d207-a263-403a-b268-05772ec10757"

#define NS_IAPPLICATIONUPDATESERVICE_IID \
  {0x1107d207, 0xa263, 0x403a, \
    { 0xb2, 0x68, 0x05, 0x77, 0x2e, 0xc1, 0x07, 0x57 }}

class NS_NO_VTABLE nsIApplicationUpdateService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IAPPLICATIONUPDATESERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIApplicationUpdateService;

  /* bool checkForBackgroundUpdates (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CheckForBackgroundUpdates(bool *_retval) = 0;

  /* readonly attribute nsIUpdateChecker backgroundChecker; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetBackgroundChecker(nsIUpdateChecker **aBackgroundChecker) = 0;

  /* nsIUpdate selectUpdate (in Array<nsIUpdate> updates); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SelectUpdate(const nsTArray<RefPtr<nsIUpdate>>& updates, nsIUpdate **_retval) = 0;

  /* void addDownloadListener (in nsIRequestObserver listener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddDownloadListener(nsIRequestObserver *listener) = 0;

  /* void removeDownloadListener (in nsIRequestObserver listener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveDownloadListener(nsIRequestObserver *listener) = 0;

  /* bool downloadUpdate (in nsIUpdate update, in boolean background); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DownloadUpdate(nsIUpdate *update, bool background, bool *_retval) = 0;

  /* void stopDownload (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD StopDownload(void) = 0;

  /* readonly attribute boolean isDownloading; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsDownloading(bool *aIsDownloading) = 0;

  /* readonly attribute boolean canCheckForUpdates; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCanCheckForUpdates(bool *aCanCheckForUpdates) = 0;

  /* readonly attribute boolean elevationRequired; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetElevationRequired(bool *aElevationRequired) = 0;

  /* readonly attribute boolean canApplyUpdates; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCanApplyUpdates(bool *aCanApplyUpdates) = 0;

  /* readonly attribute boolean isOtherInstanceHandlingUpdates; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsOtherInstanceHandlingUpdates(bool *aIsOtherInstanceHandlingUpdates) = 0;

  /* readonly attribute boolean canStageUpdates; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCanStageUpdates(bool *aCanStageUpdates) = 0;

  /* readonly attribute boolean manualUpdateOnly; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetManualUpdateOnly(bool *aManualUpdateOnly) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIApplicationUpdateService, NS_IAPPLICATIONUPDATESERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIAPPLICATIONUPDATESERVICE \
  NS_IMETHOD CheckForBackgroundUpdates(bool *_retval) override; \
  NS_IMETHOD GetBackgroundChecker(nsIUpdateChecker **aBackgroundChecker) override; \
  NS_IMETHOD SelectUpdate(const nsTArray<RefPtr<nsIUpdate>>& updates, nsIUpdate **_retval) override; \
  NS_IMETHOD AddDownloadListener(nsIRequestObserver *listener) override; \
  NS_IMETHOD RemoveDownloadListener(nsIRequestObserver *listener) override; \
  NS_IMETHOD DownloadUpdate(nsIUpdate *update, bool background, bool *_retval) override; \
  NS_IMETHOD StopDownload(void) override; \
  NS_IMETHOD GetIsDownloading(bool *aIsDownloading) override; \
  NS_IMETHOD GetCanCheckForUpdates(bool *aCanCheckForUpdates) override; \
  NS_IMETHOD GetElevationRequired(bool *aElevationRequired) override; \
  NS_IMETHOD GetCanApplyUpdates(bool *aCanApplyUpdates) override; \
  NS_IMETHOD GetIsOtherInstanceHandlingUpdates(bool *aIsOtherInstanceHandlingUpdates) override; \
  NS_IMETHOD GetCanStageUpdates(bool *aCanStageUpdates) override; \
  NS_IMETHOD GetManualUpdateOnly(bool *aManualUpdateOnly) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIAPPLICATIONUPDATESERVICE \
  nsresult CheckForBackgroundUpdates(bool *_retval); \
  nsresult GetBackgroundChecker(nsIUpdateChecker **aBackgroundChecker); \
  nsresult SelectUpdate(const nsTArray<RefPtr<nsIUpdate>>& updates, nsIUpdate **_retval); \
  nsresult AddDownloadListener(nsIRequestObserver *listener); \
  nsresult RemoveDownloadListener(nsIRequestObserver *listener); \
  nsresult DownloadUpdate(nsIUpdate *update, bool background, bool *_retval); \
  nsresult StopDownload(void); \
  nsresult GetIsDownloading(bool *aIsDownloading); \
  nsresult GetCanCheckForUpdates(bool *aCanCheckForUpdates); \
  nsresult GetElevationRequired(bool *aElevationRequired); \
  nsresult GetCanApplyUpdates(bool *aCanApplyUpdates); \
  nsresult GetIsOtherInstanceHandlingUpdates(bool *aIsOtherInstanceHandlingUpdates); \
  nsresult GetCanStageUpdates(bool *aCanStageUpdates); \
  nsresult GetManualUpdateOnly(bool *aManualUpdateOnly); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIAPPLICATIONUPDATESERVICE(_to) \
  NS_IMETHOD CheckForBackgroundUpdates(bool *_retval) override { return _to CheckForBackgroundUpdates(_retval); } \
  NS_IMETHOD GetBackgroundChecker(nsIUpdateChecker **aBackgroundChecker) override { return _to GetBackgroundChecker(aBackgroundChecker); } \
  NS_IMETHOD SelectUpdate(const nsTArray<RefPtr<nsIUpdate>>& updates, nsIUpdate **_retval) override { return _to SelectUpdate(updates, _retval); } \
  NS_IMETHOD AddDownloadListener(nsIRequestObserver *listener) override { return _to AddDownloadListener(listener); } \
  NS_IMETHOD RemoveDownloadListener(nsIRequestObserver *listener) override { return _to RemoveDownloadListener(listener); } \
  NS_IMETHOD DownloadUpdate(nsIUpdate *update, bool background, bool *_retval) override { return _to DownloadUpdate(update, background, _retval); } \
  NS_IMETHOD StopDownload(void) override { return _to StopDownload(); } \
  NS_IMETHOD GetIsDownloading(bool *aIsDownloading) override { return _to GetIsDownloading(aIsDownloading); } \
  NS_IMETHOD GetCanCheckForUpdates(bool *aCanCheckForUpdates) override { return _to GetCanCheckForUpdates(aCanCheckForUpdates); } \
  NS_IMETHOD GetElevationRequired(bool *aElevationRequired) override { return _to GetElevationRequired(aElevationRequired); } \
  NS_IMETHOD GetCanApplyUpdates(bool *aCanApplyUpdates) override { return _to GetCanApplyUpdates(aCanApplyUpdates); } \
  NS_IMETHOD GetIsOtherInstanceHandlingUpdates(bool *aIsOtherInstanceHandlingUpdates) override { return _to GetIsOtherInstanceHandlingUpdates(aIsOtherInstanceHandlingUpdates); } \
  NS_IMETHOD GetCanStageUpdates(bool *aCanStageUpdates) override { return _to GetCanStageUpdates(aCanStageUpdates); } \
  NS_IMETHOD GetManualUpdateOnly(bool *aManualUpdateOnly) override { return _to GetManualUpdateOnly(aManualUpdateOnly); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIAPPLICATIONUPDATESERVICE(_to) \
  NS_IMETHOD CheckForBackgroundUpdates(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CheckForBackgroundUpdates(_retval); } \
  NS_IMETHOD GetBackgroundChecker(nsIUpdateChecker **aBackgroundChecker) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBackgroundChecker(aBackgroundChecker); } \
  NS_IMETHOD SelectUpdate(const nsTArray<RefPtr<nsIUpdate>>& updates, nsIUpdate **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SelectUpdate(updates, _retval); } \
  NS_IMETHOD AddDownloadListener(nsIRequestObserver *listener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddDownloadListener(listener); } \
  NS_IMETHOD RemoveDownloadListener(nsIRequestObserver *listener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveDownloadListener(listener); } \
  NS_IMETHOD DownloadUpdate(nsIUpdate *update, bool background, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DownloadUpdate(update, background, _retval); } \
  NS_IMETHOD StopDownload(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StopDownload(); } \
  NS_IMETHOD GetIsDownloading(bool *aIsDownloading) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsDownloading(aIsDownloading); } \
  NS_IMETHOD GetCanCheckForUpdates(bool *aCanCheckForUpdates) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCanCheckForUpdates(aCanCheckForUpdates); } \
  NS_IMETHOD GetElevationRequired(bool *aElevationRequired) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetElevationRequired(aElevationRequired); } \
  NS_IMETHOD GetCanApplyUpdates(bool *aCanApplyUpdates) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCanApplyUpdates(aCanApplyUpdates); } \
  NS_IMETHOD GetIsOtherInstanceHandlingUpdates(bool *aIsOtherInstanceHandlingUpdates) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsOtherInstanceHandlingUpdates(aIsOtherInstanceHandlingUpdates); } \
  NS_IMETHOD GetCanStageUpdates(bool *aCanStageUpdates) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCanStageUpdates(aCanStageUpdates); } \
  NS_IMETHOD GetManualUpdateOnly(bool *aManualUpdateOnly) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetManualUpdateOnly(aManualUpdateOnly); } 


/* starting interface:    nsIUpdateProcessor */
#define NS_IUPDATEPROCESSOR_IID_STR "74439497-d796-4915-8cef-3dfe43027e4d"

#define NS_IUPDATEPROCESSOR_IID \
  {0x74439497, 0xd796, 0x4915, \
    { 0x8c, 0xef, 0x3d, 0xfe, 0x43, 0x02, 0x7e, 0x4d }}

class NS_NO_VTABLE nsIUpdateProcessor : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IUPDATEPROCESSOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUpdateProcessor;

  /* void processUpdate (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ProcessUpdate(void) = 0;

  /* void fixUpdateDirectoryPerms (in boolean useServiceOnFailure); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FixUpdateDirectoryPerms(bool useServiceOnFailure) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUpdateProcessor, NS_IUPDATEPROCESSOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIUPDATEPROCESSOR \
  NS_IMETHOD ProcessUpdate(void) override; \
  NS_IMETHOD FixUpdateDirectoryPerms(bool useServiceOnFailure) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIUPDATEPROCESSOR \
  nsresult ProcessUpdate(void); \
  nsresult FixUpdateDirectoryPerms(bool useServiceOnFailure); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIUPDATEPROCESSOR(_to) \
  NS_IMETHOD ProcessUpdate(void) override { return _to ProcessUpdate(); } \
  NS_IMETHOD FixUpdateDirectoryPerms(bool useServiceOnFailure) override { return _to FixUpdateDirectoryPerms(useServiceOnFailure); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIUPDATEPROCESSOR(_to) \
  NS_IMETHOD ProcessUpdate(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ProcessUpdate(); } \
  NS_IMETHOD FixUpdateDirectoryPerms(bool useServiceOnFailure) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FixUpdateDirectoryPerms(useServiceOnFailure); } 


/* starting interface:    nsIUpdateSyncManager */
#define NS_IUPDATESYNCMANAGER_IID_STR "cf4c4487-66d9-4e18-a2e9-39002245332f"

#define NS_IUPDATESYNCMANAGER_IID \
  {0xcf4c4487, 0x66d9, 0x4e18, \
    { 0xa2, 0xe9, 0x39, 0x00, 0x22, 0x45, 0x33, 0x2f }}

class NS_NO_VTABLE nsIUpdateSyncManager : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IUPDATESYNCMANAGER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUpdateSyncManager;

  /* bool isOtherInstanceRunning (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsOtherInstanceRunning(bool *_retval) = 0;

  /* void resetLock (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ResetLock(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUpdateSyncManager, NS_IUPDATESYNCMANAGER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIUPDATESYNCMANAGER \
  NS_IMETHOD IsOtherInstanceRunning(bool *_retval) override; \
  NS_IMETHOD ResetLock(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIUPDATESYNCMANAGER \
  nsresult IsOtherInstanceRunning(bool *_retval); \
  nsresult ResetLock(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIUPDATESYNCMANAGER(_to) \
  NS_IMETHOD IsOtherInstanceRunning(bool *_retval) override { return _to IsOtherInstanceRunning(_retval); } \
  NS_IMETHOD ResetLock(void) override { return _to ResetLock(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIUPDATESYNCMANAGER(_to) \
  NS_IMETHOD IsOtherInstanceRunning(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsOtherInstanceRunning(_retval); } \
  NS_IMETHOD ResetLock(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResetLock(); } 


/* starting interface:    nsIUpdateManager */
#define NS_IUPDATEMANAGER_IID_STR "0f1098e9-a447-4af9-b030-6f8f35c85f89"

#define NS_IUPDATEMANAGER_IID \
  {0x0f1098e9, 0xa447, 0x4af9, \
    { 0xb0, 0x30, 0x6f, 0x8f, 0x35, 0xc8, 0x5f, 0x89 }}

class NS_NO_VTABLE nsIUpdateManager : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IUPDATEMANAGER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUpdateManager;

  /* nsIUpdate getUpdateAt (in long index); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUpdateAt(int32_t index, nsIUpdate **_retval) = 0;

  /* long getUpdateCount (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUpdateCount(int32_t *_retval) = 0;

  /* attribute nsIUpdate readyUpdate; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetReadyUpdate(nsIUpdate **aReadyUpdate) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetReadyUpdate(nsIUpdate *aReadyUpdate) = 0;

  /* attribute nsIUpdate downloadingUpdate; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDownloadingUpdate(nsIUpdate **aDownloadingUpdate) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDownloadingUpdate(nsIUpdate *aDownloadingUpdate) = 0;

  /* void addUpdateToHistory (in nsIUpdate update); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddUpdateToHistory(nsIUpdate *update) = 0;

  /* void saveUpdates (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SaveUpdates(void) = 0;

  /* void refreshUpdateStatus (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RefreshUpdateStatus(void) = 0;

  /* void elevationOptedIn (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ElevationOptedIn(void) = 0;

  /* void cleanupDownloadingUpdate (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CleanupDownloadingUpdate(void) = 0;

  /* void cleanupReadyUpdate (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CleanupReadyUpdate(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUpdateManager, NS_IUPDATEMANAGER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIUPDATEMANAGER \
  NS_IMETHOD GetUpdateAt(int32_t index, nsIUpdate **_retval) override; \
  NS_IMETHOD GetUpdateCount(int32_t *_retval) override; \
  NS_IMETHOD GetReadyUpdate(nsIUpdate **aReadyUpdate) override; \
  NS_IMETHOD SetReadyUpdate(nsIUpdate *aReadyUpdate) override; \
  NS_IMETHOD GetDownloadingUpdate(nsIUpdate **aDownloadingUpdate) override; \
  NS_IMETHOD SetDownloadingUpdate(nsIUpdate *aDownloadingUpdate) override; \
  NS_IMETHOD AddUpdateToHistory(nsIUpdate *update) override; \
  NS_IMETHOD SaveUpdates(void) override; \
  NS_IMETHOD RefreshUpdateStatus(void) override; \
  NS_IMETHOD ElevationOptedIn(void) override; \
  NS_IMETHOD CleanupDownloadingUpdate(void) override; \
  NS_IMETHOD CleanupReadyUpdate(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIUPDATEMANAGER \
  nsresult GetUpdateAt(int32_t index, nsIUpdate **_retval); \
  nsresult GetUpdateCount(int32_t *_retval); \
  nsresult GetReadyUpdate(nsIUpdate **aReadyUpdate); \
  nsresult SetReadyUpdate(nsIUpdate *aReadyUpdate); \
  nsresult GetDownloadingUpdate(nsIUpdate **aDownloadingUpdate); \
  nsresult SetDownloadingUpdate(nsIUpdate *aDownloadingUpdate); \
  nsresult AddUpdateToHistory(nsIUpdate *update); \
  nsresult SaveUpdates(void); \
  nsresult RefreshUpdateStatus(void); \
  nsresult ElevationOptedIn(void); \
  nsresult CleanupDownloadingUpdate(void); \
  nsresult CleanupReadyUpdate(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIUPDATEMANAGER(_to) \
  NS_IMETHOD GetUpdateAt(int32_t index, nsIUpdate **_retval) override { return _to GetUpdateAt(index, _retval); } \
  NS_IMETHOD GetUpdateCount(int32_t *_retval) override { return _to GetUpdateCount(_retval); } \
  NS_IMETHOD GetReadyUpdate(nsIUpdate **aReadyUpdate) override { return _to GetReadyUpdate(aReadyUpdate); } \
  NS_IMETHOD SetReadyUpdate(nsIUpdate *aReadyUpdate) override { return _to SetReadyUpdate(aReadyUpdate); } \
  NS_IMETHOD GetDownloadingUpdate(nsIUpdate **aDownloadingUpdate) override { return _to GetDownloadingUpdate(aDownloadingUpdate); } \
  NS_IMETHOD SetDownloadingUpdate(nsIUpdate *aDownloadingUpdate) override { return _to SetDownloadingUpdate(aDownloadingUpdate); } \
  NS_IMETHOD AddUpdateToHistory(nsIUpdate *update) override { return _to AddUpdateToHistory(update); } \
  NS_IMETHOD SaveUpdates(void) override { return _to SaveUpdates(); } \
  NS_IMETHOD RefreshUpdateStatus(void) override { return _to RefreshUpdateStatus(); } \
  NS_IMETHOD ElevationOptedIn(void) override { return _to ElevationOptedIn(); } \
  NS_IMETHOD CleanupDownloadingUpdate(void) override { return _to CleanupDownloadingUpdate(); } \
  NS_IMETHOD CleanupReadyUpdate(void) override { return _to CleanupReadyUpdate(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIUPDATEMANAGER(_to) \
  NS_IMETHOD GetUpdateAt(int32_t index, nsIUpdate **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUpdateAt(index, _retval); } \
  NS_IMETHOD GetUpdateCount(int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUpdateCount(_retval); } \
  NS_IMETHOD GetReadyUpdate(nsIUpdate **aReadyUpdate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetReadyUpdate(aReadyUpdate); } \
  NS_IMETHOD SetReadyUpdate(nsIUpdate *aReadyUpdate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetReadyUpdate(aReadyUpdate); } \
  NS_IMETHOD GetDownloadingUpdate(nsIUpdate **aDownloadingUpdate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDownloadingUpdate(aDownloadingUpdate); } \
  NS_IMETHOD SetDownloadingUpdate(nsIUpdate *aDownloadingUpdate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDownloadingUpdate(aDownloadingUpdate); } \
  NS_IMETHOD AddUpdateToHistory(nsIUpdate *update) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddUpdateToHistory(update); } \
  NS_IMETHOD SaveUpdates(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SaveUpdates(); } \
  NS_IMETHOD RefreshUpdateStatus(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RefreshUpdateStatus(); } \
  NS_IMETHOD ElevationOptedIn(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ElevationOptedIn(); } \
  NS_IMETHOD CleanupDownloadingUpdate(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CleanupDownloadingUpdate(); } \
  NS_IMETHOD CleanupReadyUpdate(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CleanupReadyUpdate(); } 


#endif /* __gen_nsIUpdateService_h__ */
