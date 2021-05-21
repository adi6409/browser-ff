/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/browser/components/migration/nsIBrowserProfileMigrator.idl
 */

#ifndef __gen_nsIBrowserProfileMigrator_h__
#define __gen_nsIBrowserProfileMigrator_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIArray; /* forward declaration */

class nsIProfileStartup; /* forward declaration */


/* starting interface:    nsIBrowserProfileMigrator */
#define NS_IBROWSERPROFILEMIGRATOR_IID_STR "22b56ffc-3149-43c5-b5a9-b3a6b678de93"

#define NS_IBROWSERPROFILEMIGRATOR_IID \
  {0x22b56ffc, 0x3149, 0x43c5, \
    { 0xb5, 0xa9, 0xb3, 0xa6, 0xb6, 0x78, 0xde, 0x93 }}

class NS_NO_VTABLE nsIBrowserProfileMigrator : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IBROWSERPROFILEMIGRATOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIBrowserProfileMigrator;

  enum {
    ALL = 0U,
    COOKIES = 2U,
    HISTORY = 4U,
    FORMDATA = 8U,
    PASSWORDS = 16U,
    BOOKMARKS = 32U,
    OTHERDATA = 64U,
    SESSION = 128U
  };

  /* void migrate (in unsigned short aItems, in nsIProfileStartup aStartup, in jsval aProfile); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Migrate(uint16_t aItems, nsIProfileStartup *aStartup, JS::HandleValue aProfile) = 0;

  /* jsval getMigrateData (in jsval aProfile); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMigrateData(JS::HandleValue aProfile, JS::MutableHandleValue _retval) = 0;

  /* jsval getLastUsedDate (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLastUsedDate(JS::MutableHandleValue _retval) = 0;

  /* jsval isSourceAvailable (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsSourceAvailable(JS::MutableHandleValue _retval) = 0;

  /* jsval getSourceProfiles (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSourceProfiles(JS::MutableHandleValue _retval) = 0;

  /* readonly attribute boolean sourceLocked; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSourceLocked(bool *aSourceLocked) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIBrowserProfileMigrator, NS_IBROWSERPROFILEMIGRATOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIBROWSERPROFILEMIGRATOR \
  NS_IMETHOD Migrate(uint16_t aItems, nsIProfileStartup *aStartup, JS::HandleValue aProfile) override; \
  NS_IMETHOD GetMigrateData(JS::HandleValue aProfile, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetLastUsedDate(JS::MutableHandleValue _retval) override; \
  NS_IMETHOD IsSourceAvailable(JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetSourceProfiles(JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetSourceLocked(bool *aSourceLocked) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIBROWSERPROFILEMIGRATOR \
  nsresult Migrate(uint16_t aItems, nsIProfileStartup *aStartup, JS::HandleValue aProfile); \
  nsresult GetMigrateData(JS::HandleValue aProfile, JS::MutableHandleValue _retval); \
  nsresult GetLastUsedDate(JS::MutableHandleValue _retval); \
  nsresult IsSourceAvailable(JS::MutableHandleValue _retval); \
  nsresult GetSourceProfiles(JS::MutableHandleValue _retval); \
  nsresult GetSourceLocked(bool *aSourceLocked); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIBROWSERPROFILEMIGRATOR(_to) \
  NS_IMETHOD Migrate(uint16_t aItems, nsIProfileStartup *aStartup, JS::HandleValue aProfile) override { return _to Migrate(aItems, aStartup, aProfile); } \
  NS_IMETHOD GetMigrateData(JS::HandleValue aProfile, JS::MutableHandleValue _retval) override { return _to GetMigrateData(aProfile, _retval); } \
  NS_IMETHOD GetLastUsedDate(JS::MutableHandleValue _retval) override { return _to GetLastUsedDate(_retval); } \
  NS_IMETHOD IsSourceAvailable(JS::MutableHandleValue _retval) override { return _to IsSourceAvailable(_retval); } \
  NS_IMETHOD GetSourceProfiles(JS::MutableHandleValue _retval) override { return _to GetSourceProfiles(_retval); } \
  NS_IMETHOD GetSourceLocked(bool *aSourceLocked) override { return _to GetSourceLocked(aSourceLocked); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIBROWSERPROFILEMIGRATOR(_to) \
  NS_IMETHOD Migrate(uint16_t aItems, nsIProfileStartup *aStartup, JS::HandleValue aProfile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Migrate(aItems, aStartup, aProfile); } \
  NS_IMETHOD GetMigrateData(JS::HandleValue aProfile, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMigrateData(aProfile, _retval); } \
  NS_IMETHOD GetLastUsedDate(JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastUsedDate(_retval); } \
  NS_IMETHOD IsSourceAvailable(JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsSourceAvailable(_retval); } \
  NS_IMETHOD GetSourceProfiles(JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSourceProfiles(_retval); } \
  NS_IMETHOD GetSourceLocked(bool *aSourceLocked) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSourceLocked(aSourceLocked); } 


#endif /* __gen_nsIBrowserProfileMigrator_h__ */
