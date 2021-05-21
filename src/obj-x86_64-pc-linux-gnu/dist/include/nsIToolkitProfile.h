/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/profile/nsIToolkitProfile.idl
 */

#ifndef __gen_nsIToolkitProfile_h__
#define __gen_nsIToolkitProfile_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIFile; /* forward declaration */

class nsIProfileUnlocker; /* forward declaration */


/* starting interface:    nsIProfileLock */
#define NS_IPROFILELOCK_IID_STR "7c58c703-d245-4864-8d75-9648ca4a6139"

#define NS_IPROFILELOCK_IID \
  {0x7c58c703, 0xd245, 0x4864, \
    { 0x8d, 0x75, 0x96, 0x48, 0xca, 0x4a, 0x61, 0x39 }}

class NS_NO_VTABLE nsIProfileLock : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPROFILELOCK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIProfileLock;

  /* readonly attribute nsIFile directory; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDirectory(nsIFile **aDirectory) = 0;

  /* readonly attribute nsIFile localDirectory; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLocalDirectory(nsIFile **aLocalDirectory) = 0;

  /* readonly attribute PRTime replacedLockTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetReplacedLockTime(PRTime *aReplacedLockTime) = 0;

  /* void unlock (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Unlock(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIProfileLock, NS_IPROFILELOCK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPROFILELOCK \
  NS_IMETHOD GetDirectory(nsIFile **aDirectory) override; \
  NS_IMETHOD GetLocalDirectory(nsIFile **aLocalDirectory) override; \
  NS_IMETHOD GetReplacedLockTime(PRTime *aReplacedLockTime) override; \
  NS_IMETHOD Unlock(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPROFILELOCK \
  nsresult GetDirectory(nsIFile **aDirectory); \
  nsresult GetLocalDirectory(nsIFile **aLocalDirectory); \
  nsresult GetReplacedLockTime(PRTime *aReplacedLockTime); \
  nsresult Unlock(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPROFILELOCK(_to) \
  NS_IMETHOD GetDirectory(nsIFile **aDirectory) override { return _to GetDirectory(aDirectory); } \
  NS_IMETHOD GetLocalDirectory(nsIFile **aLocalDirectory) override { return _to GetLocalDirectory(aLocalDirectory); } \
  NS_IMETHOD GetReplacedLockTime(PRTime *aReplacedLockTime) override { return _to GetReplacedLockTime(aReplacedLockTime); } \
  NS_IMETHOD Unlock(void) override { return _to Unlock(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPROFILELOCK(_to) \
  NS_IMETHOD GetDirectory(nsIFile **aDirectory) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDirectory(aDirectory); } \
  NS_IMETHOD GetLocalDirectory(nsIFile **aLocalDirectory) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLocalDirectory(aLocalDirectory); } \
  NS_IMETHOD GetReplacedLockTime(PRTime *aReplacedLockTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetReplacedLockTime(aReplacedLockTime); } \
  NS_IMETHOD Unlock(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Unlock(); } 


/* starting interface:    nsIToolkitProfile */
#define NS_ITOOLKITPROFILE_IID_STR "7422b090-4a86-4407-972e-75468a625388"

#define NS_ITOOLKITPROFILE_IID \
  {0x7422b090, 0x4a86, 0x4407, \
    { 0x97, 0x2e, 0x75, 0x46, 0x8a, 0x62, 0x53, 0x88 }}

class NS_NO_VTABLE nsIToolkitProfile : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITOOLKITPROFILE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIToolkitProfile;

  /* readonly attribute nsIFile rootDir; */
  NS_IMETHOD GetRootDir(nsIFile **aRootDir) = 0;

  /* readonly attribute nsIFile localDir; */
  NS_IMETHOD GetLocalDir(nsIFile **aLocalDir) = 0;

  /* attribute AUTF8String name; */
  NS_IMETHOD GetName(nsACString& aName) = 0;
  NS_IMETHOD SetName(const nsACString& aName) = 0;

  /* void remove (in boolean removeFiles); */
  NS_IMETHOD Remove(bool removeFiles) = 0;

  /* void removeInBackground (in boolean removeFiles); */
  NS_IMETHOD RemoveInBackground(bool removeFiles) = 0;

  /* nsIProfileLock lock (out nsIProfileUnlocker aUnlocker); */
  NS_IMETHOD Lock(nsIProfileUnlocker **aUnlocker, nsIProfileLock **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIToolkitProfile, NS_ITOOLKITPROFILE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITOOLKITPROFILE \
  NS_IMETHOD GetRootDir(nsIFile **aRootDir) override; \
  NS_IMETHOD GetLocalDir(nsIFile **aLocalDir) override; \
  NS_IMETHOD GetName(nsACString& aName) override; \
  NS_IMETHOD SetName(const nsACString& aName) override; \
  NS_IMETHOD Remove(bool removeFiles) override; \
  NS_IMETHOD RemoveInBackground(bool removeFiles) override; \
  NS_IMETHOD Lock(nsIProfileUnlocker **aUnlocker, nsIProfileLock **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITOOLKITPROFILE \
  nsresult GetRootDir(nsIFile **aRootDir); \
  nsresult GetLocalDir(nsIFile **aLocalDir); \
  nsresult GetName(nsACString& aName); \
  nsresult SetName(const nsACString& aName); \
  nsresult Remove(bool removeFiles); \
  nsresult RemoveInBackground(bool removeFiles); \
  nsresult Lock(nsIProfileUnlocker **aUnlocker, nsIProfileLock **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITOOLKITPROFILE(_to) \
  NS_IMETHOD GetRootDir(nsIFile **aRootDir) override { return _to GetRootDir(aRootDir); } \
  NS_IMETHOD GetLocalDir(nsIFile **aLocalDir) override { return _to GetLocalDir(aLocalDir); } \
  NS_IMETHOD GetName(nsACString& aName) override { return _to GetName(aName); } \
  NS_IMETHOD SetName(const nsACString& aName) override { return _to SetName(aName); } \
  NS_IMETHOD Remove(bool removeFiles) override { return _to Remove(removeFiles); } \
  NS_IMETHOD RemoveInBackground(bool removeFiles) override { return _to RemoveInBackground(removeFiles); } \
  NS_IMETHOD Lock(nsIProfileUnlocker **aUnlocker, nsIProfileLock **_retval) override { return _to Lock(aUnlocker, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITOOLKITPROFILE(_to) \
  NS_IMETHOD GetRootDir(nsIFile **aRootDir) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRootDir(aRootDir); } \
  NS_IMETHOD GetLocalDir(nsIFile **aLocalDir) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLocalDir(aLocalDir); } \
  NS_IMETHOD GetName(nsACString& aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetName(aName); } \
  NS_IMETHOD SetName(const nsACString& aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetName(aName); } \
  NS_IMETHOD Remove(bool removeFiles) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Remove(removeFiles); } \
  NS_IMETHOD RemoveInBackground(bool removeFiles) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveInBackground(removeFiles); } \
  NS_IMETHOD Lock(nsIProfileUnlocker **aUnlocker, nsIProfileLock **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Lock(aUnlocker, _retval); } 


#endif /* __gen_nsIToolkitProfile_h__ */
