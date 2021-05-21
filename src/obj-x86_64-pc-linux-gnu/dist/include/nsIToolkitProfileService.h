/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/profile/nsIToolkitProfileService.idl
 */

#ifndef __gen_nsIToolkitProfileService_h__
#define __gen_nsIToolkitProfileService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "mozilla/AlreadyAddRefed.h"
#include "mozilla/Assertions.h"
#include "mozilla/DebugOnly.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsISimpleEnumerator; /* forward declaration */

class nsIFile; /* forward declaration */

class nsIToolkitProfile; /* forward declaration */

class nsIProfileLock; /* forward declaration */


/* starting interface:    nsIToolkitProfileService */
#define NS_ITOOLKITPROFILESERVICE_IID_STR "1947899b-f369-48fa-89da-f7c37bb1e6bc"

#define NS_ITOOLKITPROFILESERVICE_IID \
  {0x1947899b, 0xf369, 0x48fa, \
    { 0x89, 0xda, 0xf7, 0xc3, 0x7b, 0xb1, 0xe6, 0xbc }}

class NS_NO_VTABLE nsIToolkitProfileService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITOOLKITPROFILESERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIToolkitProfileService;

  /* [infallible] readonly attribute boolean isListOutdated; */
  NS_IMETHOD GetIsListOutdated(bool *aIsListOutdated) = 0;
  inline bool  GetIsListOutdated()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsListOutdated(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  enum downgradeUIFlags : uint8_t {
    hasSync = 1,
  };

  enum downgradeUIChoice : uint8_t {
    quit = 0,
    createNewProfile = 1,
  };

  enum profileManagerResult : uint8_t {
    exit = 0,
    launchWithProfile = 1,
    restart = 2,
  };

  /* attribute boolean startWithLastProfile; */
  NS_IMETHOD GetStartWithLastProfile(bool *aStartWithLastProfile) = 0;
  NS_IMETHOD SetStartWithLastProfile(bool aStartWithLastProfile) = 0;

  /* readonly attribute nsISimpleEnumerator profiles; */
  NS_IMETHOD GetProfiles(nsISimpleEnumerator **aProfiles) = 0;

  /* readonly attribute nsIToolkitProfile currentProfile; */
  NS_IMETHOD GetCurrentProfile(nsIToolkitProfile **aCurrentProfile) = 0;

  /* attribute nsIToolkitProfile defaultProfile; */
  NS_IMETHOD GetDefaultProfile(nsIToolkitProfile **aDefaultProfile) = 0;
  NS_IMETHOD SetDefaultProfile(nsIToolkitProfile *aDefaultProfile) = 0;

  /* readonly attribute boolean createdAlternateProfile; */
  NS_IMETHOD GetCreatedAlternateProfile(bool *aCreatedAlternateProfile) = 0;

  /* bool selectStartupProfile (in Array<ACString> aArgv, in boolean aIsResetting, in AUTF8String aUpdateChannel, in AUTF8String aLegacyInstallHash, out nsIFile aRootDir, out nsIFile aLocalDir, out nsIToolkitProfile aProfile); */
  NS_IMETHOD SelectStartupProfile(const nsTArray<nsCString >& aArgv, bool aIsResetting, const nsACString& aUpdateChannel, const nsACString& aLegacyInstallHash, nsIFile **aRootDir, nsIFile **aLocalDir, nsIToolkitProfile **aProfile, bool *_retval) = 0;

  /* nsIToolkitProfile getProfileByName (in AUTF8String aName); */
  NS_IMETHOD GetProfileByName(const nsACString& aName, nsIToolkitProfile **_retval) = 0;

  /* nsIToolkitProfile createProfile (in nsIFile aRootDir, in AUTF8String aName); */
  NS_IMETHOD CreateProfile(nsIFile *aRootDir, const nsACString& aName, nsIToolkitProfile **_retval) = 0;

  /* nsIToolkitProfile createUniqueProfile (in nsIFile aRootDir, in AUTF8String aNamePrefix); */
  NS_IMETHOD CreateUniqueProfile(nsIFile *aRootDir, const nsACString& aNamePrefix, nsIToolkitProfile **_retval) = 0;

  /* readonly attribute unsigned long profileCount; */
  NS_IMETHOD GetProfileCount(uint32_t *aProfileCount) = 0;

  /* void flush (); */
  NS_IMETHOD Flush(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIToolkitProfileService, NS_ITOOLKITPROFILESERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITOOLKITPROFILESERVICE \
  using nsIToolkitProfileService::GetIsListOutdated; \
  NS_IMETHOD GetIsListOutdated(bool *aIsListOutdated) override; \
  NS_IMETHOD GetStartWithLastProfile(bool *aStartWithLastProfile) override; \
  NS_IMETHOD SetStartWithLastProfile(bool aStartWithLastProfile) override; \
  NS_IMETHOD GetProfiles(nsISimpleEnumerator **aProfiles) override; \
  NS_IMETHOD GetCurrentProfile(nsIToolkitProfile **aCurrentProfile) override; \
  NS_IMETHOD GetDefaultProfile(nsIToolkitProfile **aDefaultProfile) override; \
  NS_IMETHOD SetDefaultProfile(nsIToolkitProfile *aDefaultProfile) override; \
  NS_IMETHOD GetCreatedAlternateProfile(bool *aCreatedAlternateProfile) override; \
  NS_IMETHOD SelectStartupProfile(const nsTArray<nsCString >& aArgv, bool aIsResetting, const nsACString& aUpdateChannel, const nsACString& aLegacyInstallHash, nsIFile **aRootDir, nsIFile **aLocalDir, nsIToolkitProfile **aProfile, bool *_retval) override; \
  NS_IMETHOD GetProfileByName(const nsACString& aName, nsIToolkitProfile **_retval) override; \
  NS_IMETHOD CreateProfile(nsIFile *aRootDir, const nsACString& aName, nsIToolkitProfile **_retval) override; \
  NS_IMETHOD CreateUniqueProfile(nsIFile *aRootDir, const nsACString& aNamePrefix, nsIToolkitProfile **_retval) override; \
  NS_IMETHOD GetProfileCount(uint32_t *aProfileCount) override; \
  NS_IMETHOD Flush(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITOOLKITPROFILESERVICE \
  using nsIToolkitProfileService::GetIsListOutdated; \
  nsresult GetIsListOutdated(bool *aIsListOutdated); \
  nsresult GetStartWithLastProfile(bool *aStartWithLastProfile); \
  nsresult SetStartWithLastProfile(bool aStartWithLastProfile); \
  nsresult GetProfiles(nsISimpleEnumerator **aProfiles); \
  nsresult GetCurrentProfile(nsIToolkitProfile **aCurrentProfile); \
  nsresult GetDefaultProfile(nsIToolkitProfile **aDefaultProfile); \
  nsresult SetDefaultProfile(nsIToolkitProfile *aDefaultProfile); \
  nsresult GetCreatedAlternateProfile(bool *aCreatedAlternateProfile); \
  nsresult SelectStartupProfile(const nsTArray<nsCString >& aArgv, bool aIsResetting, const nsACString& aUpdateChannel, const nsACString& aLegacyInstallHash, nsIFile **aRootDir, nsIFile **aLocalDir, nsIToolkitProfile **aProfile, bool *_retval); \
  nsresult GetProfileByName(const nsACString& aName, nsIToolkitProfile **_retval); \
  nsresult CreateProfile(nsIFile *aRootDir, const nsACString& aName, nsIToolkitProfile **_retval); \
  nsresult CreateUniqueProfile(nsIFile *aRootDir, const nsACString& aNamePrefix, nsIToolkitProfile **_retval); \
  nsresult GetProfileCount(uint32_t *aProfileCount); \
  nsresult Flush(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITOOLKITPROFILESERVICE(_to) \
  using nsIToolkitProfileService::GetIsListOutdated; \
  NS_IMETHOD GetIsListOutdated(bool *aIsListOutdated) override { return _to GetIsListOutdated(aIsListOutdated); } \
  NS_IMETHOD GetStartWithLastProfile(bool *aStartWithLastProfile) override { return _to GetStartWithLastProfile(aStartWithLastProfile); } \
  NS_IMETHOD SetStartWithLastProfile(bool aStartWithLastProfile) override { return _to SetStartWithLastProfile(aStartWithLastProfile); } \
  NS_IMETHOD GetProfiles(nsISimpleEnumerator **aProfiles) override { return _to GetProfiles(aProfiles); } \
  NS_IMETHOD GetCurrentProfile(nsIToolkitProfile **aCurrentProfile) override { return _to GetCurrentProfile(aCurrentProfile); } \
  NS_IMETHOD GetDefaultProfile(nsIToolkitProfile **aDefaultProfile) override { return _to GetDefaultProfile(aDefaultProfile); } \
  NS_IMETHOD SetDefaultProfile(nsIToolkitProfile *aDefaultProfile) override { return _to SetDefaultProfile(aDefaultProfile); } \
  NS_IMETHOD GetCreatedAlternateProfile(bool *aCreatedAlternateProfile) override { return _to GetCreatedAlternateProfile(aCreatedAlternateProfile); } \
  NS_IMETHOD SelectStartupProfile(const nsTArray<nsCString >& aArgv, bool aIsResetting, const nsACString& aUpdateChannel, const nsACString& aLegacyInstallHash, nsIFile **aRootDir, nsIFile **aLocalDir, nsIToolkitProfile **aProfile, bool *_retval) override { return _to SelectStartupProfile(aArgv, aIsResetting, aUpdateChannel, aLegacyInstallHash, aRootDir, aLocalDir, aProfile, _retval); } \
  NS_IMETHOD GetProfileByName(const nsACString& aName, nsIToolkitProfile **_retval) override { return _to GetProfileByName(aName, _retval); } \
  NS_IMETHOD CreateProfile(nsIFile *aRootDir, const nsACString& aName, nsIToolkitProfile **_retval) override { return _to CreateProfile(aRootDir, aName, _retval); } \
  NS_IMETHOD CreateUniqueProfile(nsIFile *aRootDir, const nsACString& aNamePrefix, nsIToolkitProfile **_retval) override { return _to CreateUniqueProfile(aRootDir, aNamePrefix, _retval); } \
  NS_IMETHOD GetProfileCount(uint32_t *aProfileCount) override { return _to GetProfileCount(aProfileCount); } \
  NS_IMETHOD Flush(void) override { return _to Flush(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITOOLKITPROFILESERVICE(_to) \
  NS_IMETHOD GetIsListOutdated(bool *aIsListOutdated) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsListOutdated(aIsListOutdated); } \
  NS_IMETHOD GetStartWithLastProfile(bool *aStartWithLastProfile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStartWithLastProfile(aStartWithLastProfile); } \
  NS_IMETHOD SetStartWithLastProfile(bool aStartWithLastProfile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetStartWithLastProfile(aStartWithLastProfile); } \
  NS_IMETHOD GetProfiles(nsISimpleEnumerator **aProfiles) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProfiles(aProfiles); } \
  NS_IMETHOD GetCurrentProfile(nsIToolkitProfile **aCurrentProfile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCurrentProfile(aCurrentProfile); } \
  NS_IMETHOD GetDefaultProfile(nsIToolkitProfile **aDefaultProfile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDefaultProfile(aDefaultProfile); } \
  NS_IMETHOD SetDefaultProfile(nsIToolkitProfile *aDefaultProfile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDefaultProfile(aDefaultProfile); } \
  NS_IMETHOD GetCreatedAlternateProfile(bool *aCreatedAlternateProfile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCreatedAlternateProfile(aCreatedAlternateProfile); } \
  NS_IMETHOD SelectStartupProfile(const nsTArray<nsCString >& aArgv, bool aIsResetting, const nsACString& aUpdateChannel, const nsACString& aLegacyInstallHash, nsIFile **aRootDir, nsIFile **aLocalDir, nsIToolkitProfile **aProfile, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SelectStartupProfile(aArgv, aIsResetting, aUpdateChannel, aLegacyInstallHash, aRootDir, aLocalDir, aProfile, _retval); } \
  NS_IMETHOD GetProfileByName(const nsACString& aName, nsIToolkitProfile **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProfileByName(aName, _retval); } \
  NS_IMETHOD CreateProfile(nsIFile *aRootDir, const nsACString& aName, nsIToolkitProfile **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateProfile(aRootDir, aName, _retval); } \
  NS_IMETHOD CreateUniqueProfile(nsIFile *aRootDir, const nsACString& aNamePrefix, nsIToolkitProfile **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateUniqueProfile(aRootDir, aNamePrefix, _retval); } \
  NS_IMETHOD GetProfileCount(uint32_t *aProfileCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProfileCount(aProfileCount); } \
  NS_IMETHOD Flush(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Flush(); } 

#define NS_PROFILESERVICE_CONTRACTID "@mozilla.org/toolkit/profile-service;1"

#endif /* __gen_nsIToolkitProfileService_h__ */
