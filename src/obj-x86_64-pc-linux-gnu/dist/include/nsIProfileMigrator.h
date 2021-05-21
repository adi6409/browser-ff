/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/profile/nsIProfileMigrator.idl
 */

#ifndef __gen_nsIProfileMigrator_h__
#define __gen_nsIProfileMigrator_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIFile; /* forward declaration */


/* starting interface:    nsIProfileStartup */
#define NS_IPROFILESTARTUP_IID_STR "048e5ca1-0eb7-4bb1-a9a2-a36f7d4e0e3c"

#define NS_IPROFILESTARTUP_IID \
  {0x048e5ca1, 0x0eb7, 0x4bb1, \
    { 0xa9, 0xa2, 0xa3, 0x6f, 0x7d, 0x4e, 0x0e, 0x3c }}

class NS_NO_VTABLE nsIProfileStartup : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPROFILESTARTUP_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIProfileStartup;

  /* readonly attribute nsIFile directory; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDirectory(nsIFile **aDirectory) = 0;

  /* void doStartup (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DoStartup(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIProfileStartup, NS_IPROFILESTARTUP_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPROFILESTARTUP \
  NS_IMETHOD GetDirectory(nsIFile **aDirectory) override; \
  NS_IMETHOD DoStartup(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPROFILESTARTUP \
  nsresult GetDirectory(nsIFile **aDirectory); \
  nsresult DoStartup(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPROFILESTARTUP(_to) \
  NS_IMETHOD GetDirectory(nsIFile **aDirectory) override { return _to GetDirectory(aDirectory); } \
  NS_IMETHOD DoStartup(void) override { return _to DoStartup(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPROFILESTARTUP(_to) \
  NS_IMETHOD GetDirectory(nsIFile **aDirectory) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDirectory(aDirectory); } \
  NS_IMETHOD DoStartup(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DoStartup(); } 


/* starting interface:    nsIProfileMigrator */
#define NS_IPROFILEMIGRATOR_IID_STR "3df284a5-2258-4d46-a664-761ecdc04c22"

#define NS_IPROFILEMIGRATOR_IID \
  {0x3df284a5, 0x2258, 0x4d46, \
    { 0xa6, 0x64, 0x76, 0x1e, 0xcd, 0xc0, 0x4c, 0x22 }}

class NS_NO_VTABLE nsIProfileMigrator : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPROFILEMIGRATOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIProfileMigrator;

  /* void migrate (in nsIProfileStartup aStartup, in ACString aKey, [optional] in AUTF8String aProfileName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Migrate(nsIProfileStartup *aStartup, const nsACString& aKey, const nsACString& aProfileName) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIProfileMigrator, NS_IPROFILEMIGRATOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPROFILEMIGRATOR \
  NS_IMETHOD Migrate(nsIProfileStartup *aStartup, const nsACString& aKey, const nsACString& aProfileName) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPROFILEMIGRATOR \
  nsresult Migrate(nsIProfileStartup *aStartup, const nsACString& aKey, const nsACString& aProfileName); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPROFILEMIGRATOR(_to) \
  NS_IMETHOD Migrate(nsIProfileStartup *aStartup, const nsACString& aKey, const nsACString& aProfileName) override { return _to Migrate(aStartup, aKey, aProfileName); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPROFILEMIGRATOR(_to) \
  NS_IMETHOD Migrate(nsIProfileStartup *aStartup, const nsACString& aKey, const nsACString& aProfileName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Migrate(aStartup, aKey, aProfileName); } 

#define NS_PROFILEMIGRATOR_CONTRACTID "@mozilla.org/toolkit/profile-migrator;1"

#endif /* __gen_nsIProfileMigrator_h__ */
