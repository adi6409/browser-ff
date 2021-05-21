/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/commandhandler/nsICommandParams.idl
 */

#ifndef __gen_nsICommandParams_h__
#define __gen_nsICommandParams_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsCommandParams;

/* starting interface:    nsICommandParams */
#define NS_ICOMMANDPARAMS_IID_STR "b1fdf3c4-74e3-4f7d-a14d-2b76bcf53482"

#define NS_ICOMMANDPARAMS_IID \
  {0xb1fdf3c4, 0x74e3, 0x4f7d, \
    { 0xa1, 0x4d, 0x2b, 0x76, 0xbc, 0xf5, 0x34, 0x82 }}

class nsICommandParams : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICOMMANDPARAMS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICommandParams;

  enum {
    eNoType = 0,
    eBooleanType = 1,
    eLongType = 2,
    eDoubleType = 3,
    eWStringType = 4,
    eISupportsType = 5,
    eStringType = 6
  };

  /* short getValueType (in string name); */
  NS_IMETHOD GetValueType(const char * name, int16_t *_retval) = 0;

  /* boolean getBooleanValue (in string name); */
  NS_IMETHOD GetBooleanValue(const char * name, bool *_retval) = 0;

  /* long getLongValue (in string name); */
  NS_IMETHOD GetLongValue(const char * name, int32_t *_retval) = 0;

  /* double getDoubleValue (in string name); */
  NS_IMETHOD GetDoubleValue(const char * name, double *_retval) = 0;

  /* AString getStringValue (in string name); */
  NS_IMETHOD GetStringValue(const char * name, nsAString& _retval) = 0;

  /* ACString getCStringValue (in string name); */
  NS_IMETHOD GetCStringValue(const char * name, nsACString& _retval) = 0;

  /* nsISupports getISupportsValue (in string name); */
  NS_IMETHOD GetISupportsValue(const char * name, nsISupports **_retval) = 0;

  /* void setBooleanValue (in string name, in boolean value); */
  NS_IMETHOD SetBooleanValue(const char * name, bool value) = 0;

  /* void setLongValue (in string name, in long value); */
  NS_IMETHOD SetLongValue(const char * name, int32_t value) = 0;

  /* void setDoubleValue (in string name, in double value); */
  NS_IMETHOD SetDoubleValue(const char * name, double value) = 0;

  /* void setStringValue (in string name, in AString value); */
  NS_IMETHOD SetStringValue(const char * name, const nsAString& value) = 0;

  /* void setCStringValue (in string name, in ACString value); */
  NS_IMETHOD SetCStringValue(const char * name, const nsACString& value) = 0;

  /* void setISupportsValue (in string name, in nsISupports value); */
  NS_IMETHOD SetISupportsValue(const char * name, nsISupports *value) = 0;

  /* void removeValue (in string name); */
  NS_IMETHOD RemoveValue(const char * name) = 0;

   /**
   * In order to avoid circular dependency issues, these methods are defined
   * in nsCommandParams.h.  Consumers need to #include that header.
   */
  inline nsCommandParams* AsCommandParams();
  inline const nsCommandParams* AsCommandParams() const;
};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICommandParams, NS_ICOMMANDPARAMS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICOMMANDPARAMS \
  NS_IMETHOD GetValueType(const char * name, int16_t *_retval) override; \
  NS_IMETHOD GetBooleanValue(const char * name, bool *_retval) override; \
  NS_IMETHOD GetLongValue(const char * name, int32_t *_retval) override; \
  NS_IMETHOD GetDoubleValue(const char * name, double *_retval) override; \
  NS_IMETHOD GetStringValue(const char * name, nsAString& _retval) override; \
  NS_IMETHOD GetCStringValue(const char * name, nsACString& _retval) override; \
  NS_IMETHOD GetISupportsValue(const char * name, nsISupports **_retval) override; \
  NS_IMETHOD SetBooleanValue(const char * name, bool value) override; \
  NS_IMETHOD SetLongValue(const char * name, int32_t value) override; \
  NS_IMETHOD SetDoubleValue(const char * name, double value) override; \
  NS_IMETHOD SetStringValue(const char * name, const nsAString& value) override; \
  NS_IMETHOD SetCStringValue(const char * name, const nsACString& value) override; \
  NS_IMETHOD SetISupportsValue(const char * name, nsISupports *value) override; \
  NS_IMETHOD RemoveValue(const char * name) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICOMMANDPARAMS \
  nsresult GetValueType(const char * name, int16_t *_retval); \
  nsresult GetBooleanValue(const char * name, bool *_retval); \
  nsresult GetLongValue(const char * name, int32_t *_retval); \
  nsresult GetDoubleValue(const char * name, double *_retval); \
  nsresult GetStringValue(const char * name, nsAString& _retval); \
  nsresult GetCStringValue(const char * name, nsACString& _retval); \
  nsresult GetISupportsValue(const char * name, nsISupports **_retval); \
  nsresult SetBooleanValue(const char * name, bool value); \
  nsresult SetLongValue(const char * name, int32_t value); \
  nsresult SetDoubleValue(const char * name, double value); \
  nsresult SetStringValue(const char * name, const nsAString& value); \
  nsresult SetCStringValue(const char * name, const nsACString& value); \
  nsresult SetISupportsValue(const char * name, nsISupports *value); \
  nsresult RemoveValue(const char * name); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICOMMANDPARAMS(_to) \
  NS_IMETHOD GetValueType(const char * name, int16_t *_retval) override { return _to GetValueType(name, _retval); } \
  NS_IMETHOD GetBooleanValue(const char * name, bool *_retval) override { return _to GetBooleanValue(name, _retval); } \
  NS_IMETHOD GetLongValue(const char * name, int32_t *_retval) override { return _to GetLongValue(name, _retval); } \
  NS_IMETHOD GetDoubleValue(const char * name, double *_retval) override { return _to GetDoubleValue(name, _retval); } \
  NS_IMETHOD GetStringValue(const char * name, nsAString& _retval) override { return _to GetStringValue(name, _retval); } \
  NS_IMETHOD GetCStringValue(const char * name, nsACString& _retval) override { return _to GetCStringValue(name, _retval); } \
  NS_IMETHOD GetISupportsValue(const char * name, nsISupports **_retval) override { return _to GetISupportsValue(name, _retval); } \
  NS_IMETHOD SetBooleanValue(const char * name, bool value) override { return _to SetBooleanValue(name, value); } \
  NS_IMETHOD SetLongValue(const char * name, int32_t value) override { return _to SetLongValue(name, value); } \
  NS_IMETHOD SetDoubleValue(const char * name, double value) override { return _to SetDoubleValue(name, value); } \
  NS_IMETHOD SetStringValue(const char * name, const nsAString& value) override { return _to SetStringValue(name, value); } \
  NS_IMETHOD SetCStringValue(const char * name, const nsACString& value) override { return _to SetCStringValue(name, value); } \
  NS_IMETHOD SetISupportsValue(const char * name, nsISupports *value) override { return _to SetISupportsValue(name, value); } \
  NS_IMETHOD RemoveValue(const char * name) override { return _to RemoveValue(name); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICOMMANDPARAMS(_to) \
  NS_IMETHOD GetValueType(const char * name, int16_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetValueType(name, _retval); } \
  NS_IMETHOD GetBooleanValue(const char * name, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBooleanValue(name, _retval); } \
  NS_IMETHOD GetLongValue(const char * name, int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLongValue(name, _retval); } \
  NS_IMETHOD GetDoubleValue(const char * name, double *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDoubleValue(name, _retval); } \
  NS_IMETHOD GetStringValue(const char * name, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStringValue(name, _retval); } \
  NS_IMETHOD GetCStringValue(const char * name, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCStringValue(name, _retval); } \
  NS_IMETHOD GetISupportsValue(const char * name, nsISupports **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetISupportsValue(name, _retval); } \
  NS_IMETHOD SetBooleanValue(const char * name, bool value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetBooleanValue(name, value); } \
  NS_IMETHOD SetLongValue(const char * name, int32_t value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLongValue(name, value); } \
  NS_IMETHOD SetDoubleValue(const char * name, double value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDoubleValue(name, value); } \
  NS_IMETHOD SetStringValue(const char * name, const nsAString& value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetStringValue(name, value); } \
  NS_IMETHOD SetCStringValue(const char * name, const nsACString& value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCStringValue(name, value); } \
  NS_IMETHOD SetISupportsValue(const char * name, nsISupports *value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetISupportsValue(name, value); } \
  NS_IMETHOD RemoveValue(const char * name) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveValue(name); } \


#endif /* __gen_nsICommandParams_h__ */
