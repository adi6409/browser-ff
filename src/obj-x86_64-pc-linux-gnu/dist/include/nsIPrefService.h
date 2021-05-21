/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/modules/libpref/nsIPrefService.idl
 */

#ifndef __gen_nsIPrefService_h__
#define __gen_nsIPrefService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIPrefBranch_h__
#include "nsIPrefBranch.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIFile; /* forward declaration */


/* starting interface:    nsIPrefStatsCallback */
#define NS_IPREFSTATSCALLBACK_IID_STR "c3f0cedc-e244-4316-b33a-80306a1c35a1"

#define NS_IPREFSTATSCALLBACK_IID \
  {0xc3f0cedc, 0xe244, 0x4316, \
    { 0xb3, 0x3a, 0x80, 0x30, 0x6a, 0x1c, 0x35, 0xa1 }}

class NS_NO_VTABLE nsIPrefStatsCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPREFSTATSCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPrefStatsCallback;

  /* void visit (in ACString prefName, in unsigned long accessCount); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Visit(const nsACString& prefName, uint32_t accessCount) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPrefStatsCallback, NS_IPREFSTATSCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPREFSTATSCALLBACK \
  NS_IMETHOD Visit(const nsACString& prefName, uint32_t accessCount) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPREFSTATSCALLBACK \
  nsresult Visit(const nsACString& prefName, uint32_t accessCount); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPREFSTATSCALLBACK(_to) \
  NS_IMETHOD Visit(const nsACString& prefName, uint32_t accessCount) override { return _to Visit(prefName, accessCount); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPREFSTATSCALLBACK(_to) \
  NS_IMETHOD Visit(const nsACString& prefName, uint32_t accessCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Visit(prefName, accessCount); } 


/* starting interface:    nsIPrefService */
#define NS_IPREFSERVICE_IID_STR "1f84fd56-3956-40df-b86a-1ea01402ee96"

#define NS_IPREFSERVICE_IID \
  {0x1f84fd56, 0x3956, 0x40df, \
    { 0xb8, 0x6a, 0x1e, 0xa0, 0x14, 0x02, 0xee, 0x96 }}

class NS_NO_VTABLE nsIPrefService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPREFSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPrefService;

  /* void resetPrefs (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ResetPrefs(void) = 0;

  /* void resetUserPrefs (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ResetUserPrefs(void) = 0;

  /* void savePrefFile (in nsIFile aFile); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SavePrefFile(nsIFile *aFile) = 0;

  /* nsIPrefBranch getBranch (in string aPrefRoot); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetBranch(const char * aPrefRoot, nsIPrefBranch **_retval) = 0;

  /* nsIPrefBranch getDefaultBranch (in string aPrefRoot); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDefaultBranch(const char * aPrefRoot, nsIPrefBranch **_retval) = 0;

  /* readonly attribute boolean dirty; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDirty(bool *aDirty) = 0;

  /* void readDefaultPrefsFromFile (in nsIFile aFile); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ReadDefaultPrefsFromFile(nsIFile *aFile) = 0;

  /* void readUserPrefsFromFile (in nsIFile aFile); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ReadUserPrefsFromFile(nsIFile *aFile) = 0;

  /* void readStats (in nsIPrefStatsCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ReadStats(nsIPrefStatsCallback *callback) = 0;

  /* void resetStats (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ResetStats(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPrefService, NS_IPREFSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPREFSERVICE \
  NS_IMETHOD ResetPrefs(void) override; \
  NS_IMETHOD ResetUserPrefs(void) override; \
  NS_IMETHOD SavePrefFile(nsIFile *aFile) override; \
  NS_IMETHOD GetBranch(const char * aPrefRoot, nsIPrefBranch **_retval) override; \
  NS_IMETHOD GetDefaultBranch(const char * aPrefRoot, nsIPrefBranch **_retval) override; \
  NS_IMETHOD GetDirty(bool *aDirty) override; \
  NS_IMETHOD ReadDefaultPrefsFromFile(nsIFile *aFile) override; \
  NS_IMETHOD ReadUserPrefsFromFile(nsIFile *aFile) override; \
  NS_IMETHOD ReadStats(nsIPrefStatsCallback *callback) override; \
  NS_IMETHOD ResetStats(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPREFSERVICE \
  nsresult ResetPrefs(void); \
  nsresult ResetUserPrefs(void); \
  nsresult SavePrefFile(nsIFile *aFile); \
  nsresult GetBranch(const char * aPrefRoot, nsIPrefBranch **_retval); \
  nsresult GetDefaultBranch(const char * aPrefRoot, nsIPrefBranch **_retval); \
  nsresult GetDirty(bool *aDirty); \
  nsresult ReadDefaultPrefsFromFile(nsIFile *aFile); \
  nsresult ReadUserPrefsFromFile(nsIFile *aFile); \
  nsresult ReadStats(nsIPrefStatsCallback *callback); \
  nsresult ResetStats(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPREFSERVICE(_to) \
  NS_IMETHOD ResetPrefs(void) override { return _to ResetPrefs(); } \
  NS_IMETHOD ResetUserPrefs(void) override { return _to ResetUserPrefs(); } \
  NS_IMETHOD SavePrefFile(nsIFile *aFile) override { return _to SavePrefFile(aFile); } \
  NS_IMETHOD GetBranch(const char * aPrefRoot, nsIPrefBranch **_retval) override { return _to GetBranch(aPrefRoot, _retval); } \
  NS_IMETHOD GetDefaultBranch(const char * aPrefRoot, nsIPrefBranch **_retval) override { return _to GetDefaultBranch(aPrefRoot, _retval); } \
  NS_IMETHOD GetDirty(bool *aDirty) override { return _to GetDirty(aDirty); } \
  NS_IMETHOD ReadDefaultPrefsFromFile(nsIFile *aFile) override { return _to ReadDefaultPrefsFromFile(aFile); } \
  NS_IMETHOD ReadUserPrefsFromFile(nsIFile *aFile) override { return _to ReadUserPrefsFromFile(aFile); } \
  NS_IMETHOD ReadStats(nsIPrefStatsCallback *callback) override { return _to ReadStats(callback); } \
  NS_IMETHOD ResetStats(void) override { return _to ResetStats(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPREFSERVICE(_to) \
  NS_IMETHOD ResetPrefs(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResetPrefs(); } \
  NS_IMETHOD ResetUserPrefs(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResetUserPrefs(); } \
  NS_IMETHOD SavePrefFile(nsIFile *aFile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SavePrefFile(aFile); } \
  NS_IMETHOD GetBranch(const char * aPrefRoot, nsIPrefBranch **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBranch(aPrefRoot, _retval); } \
  NS_IMETHOD GetDefaultBranch(const char * aPrefRoot, nsIPrefBranch **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDefaultBranch(aPrefRoot, _retval); } \
  NS_IMETHOD GetDirty(bool *aDirty) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDirty(aDirty); } \
  NS_IMETHOD ReadDefaultPrefsFromFile(nsIFile *aFile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReadDefaultPrefsFromFile(aFile); } \
  NS_IMETHOD ReadUserPrefsFromFile(nsIFile *aFile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReadUserPrefsFromFile(aFile); } \
  NS_IMETHOD ReadStats(nsIPrefStatsCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReadStats(callback); } \
  NS_IMETHOD ResetStats(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResetStats(); } 


#define NS_PREFSERVICE_CID                             \
  { /* {1cd91b88-1dd2-11b2-92e1-ed22ed298000} */       \
    0x91ca2441,                                        \
    0x050f,                                            \
    0x4f7c,                                            \
    { 0x9d, 0xf8, 0x75, 0xb4, 0x0e, 0xa4, 0x01, 0x56 } \
  }
#define NS_PREFSERVICE_CONTRACTID "@mozilla.org/preferences-service;1"
/**
 * Notification sent before reading the default user preferences files.
 */
#define NS_PREFSERVICE_READ_TOPIC_ID "prefservice:before-read-userprefs"
/**
 * Notification sent when after reading app-provided default
 * preferences, but before user profile override defaults are loaded.
 */
#define NS_PREFSERVICE_APPDEFAULTS_TOPIC_ID "prefservice:after-app-defaults"

#endif /* __gen_nsIPrefService_h__ */
