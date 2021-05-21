/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/passwordmgr/nsILoginMetaInfo.idl
 */

#ifndef __gen_nsILoginMetaInfo_h__
#define __gen_nsILoginMetaInfo_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsILoginMetaInfo */
#define NS_ILOGINMETAINFO_IID_STR "20d8eb40-c494-497f-b2a6-aaa32f807ebd"

#define NS_ILOGINMETAINFO_IID \
  {0x20d8eb40, 0xc494, 0x497f, \
    { 0xb2, 0xa6, 0xaa, 0xa3, 0x2f, 0x80, 0x7e, 0xbd }}

class NS_NO_VTABLE nsILoginMetaInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ILOGINMETAINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsILoginMetaInfo;

  /* attribute AString guid; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetGuid(nsAString& aGuid) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetGuid(const nsAString& aGuid) = 0;

  /* attribute unsigned long long timeCreated; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTimeCreated(uint64_t *aTimeCreated) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetTimeCreated(uint64_t aTimeCreated) = 0;

  /* attribute unsigned long long timeLastUsed; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTimeLastUsed(uint64_t *aTimeLastUsed) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetTimeLastUsed(uint64_t aTimeLastUsed) = 0;

  /* attribute unsigned long long timePasswordChanged; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTimePasswordChanged(uint64_t *aTimePasswordChanged) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetTimePasswordChanged(uint64_t aTimePasswordChanged) = 0;

  /* attribute unsigned long timesUsed; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTimesUsed(uint32_t *aTimesUsed) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetTimesUsed(uint32_t aTimesUsed) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsILoginMetaInfo, NS_ILOGINMETAINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSILOGINMETAINFO \
  NS_IMETHOD GetGuid(nsAString& aGuid) override; \
  NS_IMETHOD SetGuid(const nsAString& aGuid) override; \
  NS_IMETHOD GetTimeCreated(uint64_t *aTimeCreated) override; \
  NS_IMETHOD SetTimeCreated(uint64_t aTimeCreated) override; \
  NS_IMETHOD GetTimeLastUsed(uint64_t *aTimeLastUsed) override; \
  NS_IMETHOD SetTimeLastUsed(uint64_t aTimeLastUsed) override; \
  NS_IMETHOD GetTimePasswordChanged(uint64_t *aTimePasswordChanged) override; \
  NS_IMETHOD SetTimePasswordChanged(uint64_t aTimePasswordChanged) override; \
  NS_IMETHOD GetTimesUsed(uint32_t *aTimesUsed) override; \
  NS_IMETHOD SetTimesUsed(uint32_t aTimesUsed) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSILOGINMETAINFO \
  nsresult GetGuid(nsAString& aGuid); \
  nsresult SetGuid(const nsAString& aGuid); \
  nsresult GetTimeCreated(uint64_t *aTimeCreated); \
  nsresult SetTimeCreated(uint64_t aTimeCreated); \
  nsresult GetTimeLastUsed(uint64_t *aTimeLastUsed); \
  nsresult SetTimeLastUsed(uint64_t aTimeLastUsed); \
  nsresult GetTimePasswordChanged(uint64_t *aTimePasswordChanged); \
  nsresult SetTimePasswordChanged(uint64_t aTimePasswordChanged); \
  nsresult GetTimesUsed(uint32_t *aTimesUsed); \
  nsresult SetTimesUsed(uint32_t aTimesUsed); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSILOGINMETAINFO(_to) \
  NS_IMETHOD GetGuid(nsAString& aGuid) override { return _to GetGuid(aGuid); } \
  NS_IMETHOD SetGuid(const nsAString& aGuid) override { return _to SetGuid(aGuid); } \
  NS_IMETHOD GetTimeCreated(uint64_t *aTimeCreated) override { return _to GetTimeCreated(aTimeCreated); } \
  NS_IMETHOD SetTimeCreated(uint64_t aTimeCreated) override { return _to SetTimeCreated(aTimeCreated); } \
  NS_IMETHOD GetTimeLastUsed(uint64_t *aTimeLastUsed) override { return _to GetTimeLastUsed(aTimeLastUsed); } \
  NS_IMETHOD SetTimeLastUsed(uint64_t aTimeLastUsed) override { return _to SetTimeLastUsed(aTimeLastUsed); } \
  NS_IMETHOD GetTimePasswordChanged(uint64_t *aTimePasswordChanged) override { return _to GetTimePasswordChanged(aTimePasswordChanged); } \
  NS_IMETHOD SetTimePasswordChanged(uint64_t aTimePasswordChanged) override { return _to SetTimePasswordChanged(aTimePasswordChanged); } \
  NS_IMETHOD GetTimesUsed(uint32_t *aTimesUsed) override { return _to GetTimesUsed(aTimesUsed); } \
  NS_IMETHOD SetTimesUsed(uint32_t aTimesUsed) override { return _to SetTimesUsed(aTimesUsed); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSILOGINMETAINFO(_to) \
  NS_IMETHOD GetGuid(nsAString& aGuid) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetGuid(aGuid); } \
  NS_IMETHOD SetGuid(const nsAString& aGuid) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetGuid(aGuid); } \
  NS_IMETHOD GetTimeCreated(uint64_t *aTimeCreated) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTimeCreated(aTimeCreated); } \
  NS_IMETHOD SetTimeCreated(uint64_t aTimeCreated) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTimeCreated(aTimeCreated); } \
  NS_IMETHOD GetTimeLastUsed(uint64_t *aTimeLastUsed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTimeLastUsed(aTimeLastUsed); } \
  NS_IMETHOD SetTimeLastUsed(uint64_t aTimeLastUsed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTimeLastUsed(aTimeLastUsed); } \
  NS_IMETHOD GetTimePasswordChanged(uint64_t *aTimePasswordChanged) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTimePasswordChanged(aTimePasswordChanged); } \
  NS_IMETHOD SetTimePasswordChanged(uint64_t aTimePasswordChanged) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTimePasswordChanged(aTimePasswordChanged); } \
  NS_IMETHOD GetTimesUsed(uint32_t *aTimesUsed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTimesUsed(aTimesUsed); } \
  NS_IMETHOD SetTimesUsed(uint32_t aTimesUsed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTimesUsed(aTimesUsed); } 


#endif /* __gen_nsILoginMetaInfo_h__ */
