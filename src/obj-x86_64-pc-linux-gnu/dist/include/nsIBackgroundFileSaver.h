/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIBackgroundFileSaver.idl
 */

#ifndef __gen_nsIBackgroundFileSaver_h__
#define __gen_nsIBackgroundFileSaver_h__


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
class nsIArray; /* forward declaration */

class nsIBackgroundFileSaverObserver; /* forward declaration */

class nsIFile; /* forward declaration */


/* starting interface:    nsIBackgroundFileSaver */
#define NS_IBACKGROUNDFILESAVER_IID_STR "c43544a4-682c-4262-b407-2453d26e660d"

#define NS_IBACKGROUNDFILESAVER_IID \
  {0xc43544a4, 0x682c, 0x4262, \
    { 0xb4, 0x07, 0x24, 0x53, 0xd2, 0x6e, 0x66, 0x0d }}

class NS_NO_VTABLE nsIBackgroundFileSaver : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IBACKGROUNDFILESAVER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIBackgroundFileSaver;

  /* attribute nsIBackgroundFileSaverObserver observer; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetObserver(nsIBackgroundFileSaverObserver **aObserver) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetObserver(nsIBackgroundFileSaverObserver *aObserver) = 0;

  /* readonly attribute Array<Array<Array<uint8_t>>> signatureInfo; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSignatureInfo(nsTArray<nsTArray<nsTArray<uint8_t >>>& aSignatureInfo) = 0;

  /* readonly attribute ACString sha256Hash; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSha256Hash(nsACString& aSha256Hash) = 0;

  /* void enableSignatureInfo (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EnableSignatureInfo(void) = 0;

  /* void enableSha256 (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EnableSha256(void) = 0;

  /* void enableAppend (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EnableAppend(void) = 0;

  /* void setTarget (in nsIFile aTarget, in bool aKeepPartial); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetTarget(nsIFile *aTarget, bool aKeepPartial) = 0;

  /* void finish (in nsresult aStatus); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Finish(nsresult aStatus) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIBackgroundFileSaver, NS_IBACKGROUNDFILESAVER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIBACKGROUNDFILESAVER \
  NS_IMETHOD GetObserver(nsIBackgroundFileSaverObserver **aObserver) override; \
  NS_IMETHOD SetObserver(nsIBackgroundFileSaverObserver *aObserver) override; \
  NS_IMETHOD GetSignatureInfo(nsTArray<nsTArray<nsTArray<uint8_t >>>& aSignatureInfo) override; \
  NS_IMETHOD GetSha256Hash(nsACString& aSha256Hash) override; \
  NS_IMETHOD EnableSignatureInfo(void) override; \
  NS_IMETHOD EnableSha256(void) override; \
  NS_IMETHOD EnableAppend(void) override; \
  NS_IMETHOD SetTarget(nsIFile *aTarget, bool aKeepPartial) override; \
  NS_IMETHOD Finish(nsresult aStatus) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIBACKGROUNDFILESAVER \
  nsresult GetObserver(nsIBackgroundFileSaverObserver **aObserver); \
  nsresult SetObserver(nsIBackgroundFileSaverObserver *aObserver); \
  nsresult GetSignatureInfo(nsTArray<nsTArray<nsTArray<uint8_t >>>& aSignatureInfo); \
  nsresult GetSha256Hash(nsACString& aSha256Hash); \
  nsresult EnableSignatureInfo(void); \
  nsresult EnableSha256(void); \
  nsresult EnableAppend(void); \
  nsresult SetTarget(nsIFile *aTarget, bool aKeepPartial); \
  nsresult Finish(nsresult aStatus); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIBACKGROUNDFILESAVER(_to) \
  NS_IMETHOD GetObserver(nsIBackgroundFileSaverObserver **aObserver) override { return _to GetObserver(aObserver); } \
  NS_IMETHOD SetObserver(nsIBackgroundFileSaverObserver *aObserver) override { return _to SetObserver(aObserver); } \
  NS_IMETHOD GetSignatureInfo(nsTArray<nsTArray<nsTArray<uint8_t >>>& aSignatureInfo) override { return _to GetSignatureInfo(aSignatureInfo); } \
  NS_IMETHOD GetSha256Hash(nsACString& aSha256Hash) override { return _to GetSha256Hash(aSha256Hash); } \
  NS_IMETHOD EnableSignatureInfo(void) override { return _to EnableSignatureInfo(); } \
  NS_IMETHOD EnableSha256(void) override { return _to EnableSha256(); } \
  NS_IMETHOD EnableAppend(void) override { return _to EnableAppend(); } \
  NS_IMETHOD SetTarget(nsIFile *aTarget, bool aKeepPartial) override { return _to SetTarget(aTarget, aKeepPartial); } \
  NS_IMETHOD Finish(nsresult aStatus) override { return _to Finish(aStatus); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIBACKGROUNDFILESAVER(_to) \
  NS_IMETHOD GetObserver(nsIBackgroundFileSaverObserver **aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetObserver(aObserver); } \
  NS_IMETHOD SetObserver(nsIBackgroundFileSaverObserver *aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetObserver(aObserver); } \
  NS_IMETHOD GetSignatureInfo(nsTArray<nsTArray<nsTArray<uint8_t >>>& aSignatureInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSignatureInfo(aSignatureInfo); } \
  NS_IMETHOD GetSha256Hash(nsACString& aSha256Hash) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSha256Hash(aSha256Hash); } \
  NS_IMETHOD EnableSignatureInfo(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnableSignatureInfo(); } \
  NS_IMETHOD EnableSha256(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnableSha256(); } \
  NS_IMETHOD EnableAppend(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnableAppend(); } \
  NS_IMETHOD SetTarget(nsIFile *aTarget, bool aKeepPartial) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTarget(aTarget, aKeepPartial); } \
  NS_IMETHOD Finish(nsresult aStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Finish(aStatus); } 


/* starting interface:    nsIBackgroundFileSaverObserver */
#define NS_IBACKGROUNDFILESAVEROBSERVER_IID_STR "ee7058c3-6e54-4411-b76b-3ce87b76fcb6"

#define NS_IBACKGROUNDFILESAVEROBSERVER_IID \
  {0xee7058c3, 0x6e54, 0x4411, \
    { 0xb7, 0x6b, 0x3c, 0xe8, 0x7b, 0x76, 0xfc, 0xb6 }}

class NS_NO_VTABLE nsIBackgroundFileSaverObserver : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IBACKGROUNDFILESAVEROBSERVER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIBackgroundFileSaverObserver;

  /* void onTargetChange (in nsIBackgroundFileSaver aSaver, in nsIFile aTarget); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnTargetChange(nsIBackgroundFileSaver *aSaver, nsIFile *aTarget) = 0;

  /* void onSaveComplete (in nsIBackgroundFileSaver aSaver, in nsresult aStatus); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnSaveComplete(nsIBackgroundFileSaver *aSaver, nsresult aStatus) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIBackgroundFileSaverObserver, NS_IBACKGROUNDFILESAVEROBSERVER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIBACKGROUNDFILESAVEROBSERVER \
  NS_IMETHOD OnTargetChange(nsIBackgroundFileSaver *aSaver, nsIFile *aTarget) override; \
  NS_IMETHOD OnSaveComplete(nsIBackgroundFileSaver *aSaver, nsresult aStatus) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIBACKGROUNDFILESAVEROBSERVER \
  nsresult OnTargetChange(nsIBackgroundFileSaver *aSaver, nsIFile *aTarget); \
  nsresult OnSaveComplete(nsIBackgroundFileSaver *aSaver, nsresult aStatus); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIBACKGROUNDFILESAVEROBSERVER(_to) \
  NS_IMETHOD OnTargetChange(nsIBackgroundFileSaver *aSaver, nsIFile *aTarget) override { return _to OnTargetChange(aSaver, aTarget); } \
  NS_IMETHOD OnSaveComplete(nsIBackgroundFileSaver *aSaver, nsresult aStatus) override { return _to OnSaveComplete(aSaver, aStatus); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIBACKGROUNDFILESAVEROBSERVER(_to) \
  NS_IMETHOD OnTargetChange(nsIBackgroundFileSaver *aSaver, nsIFile *aTarget) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnTargetChange(aSaver, aTarget); } \
  NS_IMETHOD OnSaveComplete(nsIBackgroundFileSaver *aSaver, nsresult aStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnSaveComplete(aSaver, aStatus); } 


#endif /* __gen_nsIBackgroundFileSaver_h__ */
