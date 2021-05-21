/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/services/interfaces/mozIServicesLogSink.idl
 */

#ifndef __gen_mozIServicesLogSink_h__
#define __gen_mozIServicesLogSink_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    mozIServicesLogSink */
#define MOZISERVICESLOGSINK_IID_STR "c92bfe0d-50b7-4a7f-9686-fe5335a696b9"

#define MOZISERVICESLOGSINK_IID \
  {0xc92bfe0d, 0x50b7, 0x4a7f, \
    { 0x96, 0x86, 0xfe, 0x53, 0x35, 0xa6, 0x96, 0xb9 }}

class NS_NO_VTABLE mozIServicesLogSink : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZISERVICESLOGSINK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIServicesLogSink;

  enum {
    LEVEL_OFF = 0,
    LEVEL_ERROR = 1,
    LEVEL_WARN = 2,
    LEVEL_INFO = 3,
    LEVEL_DEBUG = 4,
    LEVEL_TRACE = 5
  };

  /* attribute short maxLevel; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMaxLevel(int16_t *aMaxLevel) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetMaxLevel(int16_t aMaxLevel) = 0;

  /* void error (in AString message); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Error(const nsAString& message) = 0;

  /* void warn (in AString message); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Warn(const nsAString& message) = 0;

  /* void debug (in AString message); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Debug(const nsAString& message) = 0;

  /* void trace (in AString message); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Trace(const nsAString& message) = 0;

  /* void info (in AString message); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Info(const nsAString& message) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIServicesLogSink, MOZISERVICESLOGSINK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZISERVICESLOGSINK \
  NS_IMETHOD GetMaxLevel(int16_t *aMaxLevel) override; \
  NS_IMETHOD SetMaxLevel(int16_t aMaxLevel) override; \
  NS_IMETHOD Error(const nsAString& message) override; \
  NS_IMETHOD Warn(const nsAString& message) override; \
  NS_IMETHOD Debug(const nsAString& message) override; \
  NS_IMETHOD Trace(const nsAString& message) override; \
  NS_IMETHOD Info(const nsAString& message) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZISERVICESLOGSINK \
  nsresult GetMaxLevel(int16_t *aMaxLevel); \
  nsresult SetMaxLevel(int16_t aMaxLevel); \
  nsresult Error(const nsAString& message); \
  nsresult Warn(const nsAString& message); \
  nsresult Debug(const nsAString& message); \
  nsresult Trace(const nsAString& message); \
  nsresult Info(const nsAString& message); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZISERVICESLOGSINK(_to) \
  NS_IMETHOD GetMaxLevel(int16_t *aMaxLevel) override { return _to GetMaxLevel(aMaxLevel); } \
  NS_IMETHOD SetMaxLevel(int16_t aMaxLevel) override { return _to SetMaxLevel(aMaxLevel); } \
  NS_IMETHOD Error(const nsAString& message) override { return _to Error(message); } \
  NS_IMETHOD Warn(const nsAString& message) override { return _to Warn(message); } \
  NS_IMETHOD Debug(const nsAString& message) override { return _to Debug(message); } \
  NS_IMETHOD Trace(const nsAString& message) override { return _to Trace(message); } \
  NS_IMETHOD Info(const nsAString& message) override { return _to Info(message); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZISERVICESLOGSINK(_to) \
  NS_IMETHOD GetMaxLevel(int16_t *aMaxLevel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMaxLevel(aMaxLevel); } \
  NS_IMETHOD SetMaxLevel(int16_t aMaxLevel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMaxLevel(aMaxLevel); } \
  NS_IMETHOD Error(const nsAString& message) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Error(message); } \
  NS_IMETHOD Warn(const nsAString& message) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Warn(message); } \
  NS_IMETHOD Debug(const nsAString& message) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Debug(message); } \
  NS_IMETHOD Trace(const nsAString& message) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Trace(message); } \
  NS_IMETHOD Info(const nsAString& message) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Info(message); } 


#endif /* __gen_mozIServicesLogSink_h__ */
