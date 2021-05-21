/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/system/nsIGSettingsService.idl
 */

#ifndef __gen_nsIGSettingsService_h__
#define __gen_nsIGSettingsService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIArray; /* forward declaration */


/* starting interface:    nsIGSettingsCollection */
#define NS_IGSETTINGSCOLLECTION_IID_STR "16d5b0ed-e756-4f1b-a8ce-9132e869acd8"

#define NS_IGSETTINGSCOLLECTION_IID \
  {0x16d5b0ed, 0xe756, 0x4f1b, \
    { 0xa8, 0xce, 0x91, 0x32, 0xe8, 0x69, 0xac, 0xd8 }}

class NS_NO_VTABLE nsIGSettingsCollection : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IGSETTINGSCOLLECTION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIGSettingsCollection;

  /* void setString (in AUTF8String key, in AUTF8String value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetString(const nsACString& key, const nsACString& value) = 0;

  /* void setBoolean (in AUTF8String key, in boolean value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetBoolean(const nsACString& key, bool value) = 0;

  /* void setInt (in AUTF8String key, in long value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetInt(const nsACString& key, int32_t value) = 0;

  /* AUTF8String getString (in AUTF8String key); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetString(const nsACString& key, nsACString& _retval) = 0;

  /* boolean getBoolean (in AUTF8String key); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetBoolean(const nsACString& key, bool *_retval) = 0;

  /* long getInt (in AUTF8String key); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetInt(const nsACString& key, int32_t *_retval) = 0;

  /* nsIArray getStringList (in AUTF8String key); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetStringList(const nsACString& key, nsIArray **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIGSettingsCollection, NS_IGSETTINGSCOLLECTION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIGSETTINGSCOLLECTION \
  NS_IMETHOD SetString(const nsACString& key, const nsACString& value) override; \
  NS_IMETHOD SetBoolean(const nsACString& key, bool value) override; \
  NS_IMETHOD SetInt(const nsACString& key, int32_t value) override; \
  NS_IMETHOD GetString(const nsACString& key, nsACString& _retval) override; \
  NS_IMETHOD GetBoolean(const nsACString& key, bool *_retval) override; \
  NS_IMETHOD GetInt(const nsACString& key, int32_t *_retval) override; \
  NS_IMETHOD GetStringList(const nsACString& key, nsIArray **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIGSETTINGSCOLLECTION \
  nsresult SetString(const nsACString& key, const nsACString& value); \
  nsresult SetBoolean(const nsACString& key, bool value); \
  nsresult SetInt(const nsACString& key, int32_t value); \
  nsresult GetString(const nsACString& key, nsACString& _retval); \
  nsresult GetBoolean(const nsACString& key, bool *_retval); \
  nsresult GetInt(const nsACString& key, int32_t *_retval); \
  nsresult GetStringList(const nsACString& key, nsIArray **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIGSETTINGSCOLLECTION(_to) \
  NS_IMETHOD SetString(const nsACString& key, const nsACString& value) override { return _to SetString(key, value); } \
  NS_IMETHOD SetBoolean(const nsACString& key, bool value) override { return _to SetBoolean(key, value); } \
  NS_IMETHOD SetInt(const nsACString& key, int32_t value) override { return _to SetInt(key, value); } \
  NS_IMETHOD GetString(const nsACString& key, nsACString& _retval) override { return _to GetString(key, _retval); } \
  NS_IMETHOD GetBoolean(const nsACString& key, bool *_retval) override { return _to GetBoolean(key, _retval); } \
  NS_IMETHOD GetInt(const nsACString& key, int32_t *_retval) override { return _to GetInt(key, _retval); } \
  NS_IMETHOD GetStringList(const nsACString& key, nsIArray **_retval) override { return _to GetStringList(key, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIGSETTINGSCOLLECTION(_to) \
  NS_IMETHOD SetString(const nsACString& key, const nsACString& value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetString(key, value); } \
  NS_IMETHOD SetBoolean(const nsACString& key, bool value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetBoolean(key, value); } \
  NS_IMETHOD SetInt(const nsACString& key, int32_t value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetInt(key, value); } \
  NS_IMETHOD GetString(const nsACString& key, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetString(key, _retval); } \
  NS_IMETHOD GetBoolean(const nsACString& key, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBoolean(key, _retval); } \
  NS_IMETHOD GetInt(const nsACString& key, int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInt(key, _retval); } \
  NS_IMETHOD GetStringList(const nsACString& key, nsIArray **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStringList(key, _retval); } 


/* starting interface:    nsIGSettingsService */
#define NS_IGSETTINGSSERVICE_IID_STR "849c088b-57d1-4f24-b7b2-3dc4acb04c0a"

#define NS_IGSETTINGSSERVICE_IID \
  {0x849c088b, 0x57d1, 0x4f24, \
    { 0xb7, 0xb2, 0x3d, 0xc4, 0xac, 0xb0, 0x4c, 0x0a }}

class NS_NO_VTABLE nsIGSettingsService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IGSETTINGSSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIGSettingsService;

  /* nsIGSettingsCollection getCollectionForSchema (in AUTF8String schema); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCollectionForSchema(const nsACString& schema, nsIGSettingsCollection **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIGSettingsService, NS_IGSETTINGSSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIGSETTINGSSERVICE \
  NS_IMETHOD GetCollectionForSchema(const nsACString& schema, nsIGSettingsCollection **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIGSETTINGSSERVICE \
  nsresult GetCollectionForSchema(const nsACString& schema, nsIGSettingsCollection **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIGSETTINGSSERVICE(_to) \
  NS_IMETHOD GetCollectionForSchema(const nsACString& schema, nsIGSettingsCollection **_retval) override { return _to GetCollectionForSchema(schema, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIGSETTINGSSERVICE(_to) \
  NS_IMETHOD GetCollectionForSchema(const nsACString& schema, nsIGSettingsCollection **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCollectionForSchema(schema, _retval); } 

#define NS_GSETTINGSSERVICE_CONTRACTID "@mozilla.org/gsettings-service;1"

#endif /* __gen_nsIGSettingsService_h__ */
