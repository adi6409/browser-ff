/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/services/interfaces/mozIAppServicesLogger.idl
 */

#ifndef __gen_mozIAppServicesLogger_h__
#define __gen_mozIAppServicesLogger_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_mozIServicesLogSink_h__
#include "mozIServicesLogSink.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    mozIAppServicesLogger */
#define MOZIAPPSERVICESLOGGER_IID_STR "446dd837-fbb0-41e4-8221-f740f672b20d"

#define MOZIAPPSERVICESLOGGER_IID \
  {0x446dd837, 0xfbb0, 0x41e4, \
    { 0x82, 0x21, 0xf7, 0x40, 0xf6, 0x72, 0xb2, 0x0d }}

class NS_NO_VTABLE mozIAppServicesLogger : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZIAPPSERVICESLOGGER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIAppServicesLogger;

  /* void register (in AString target, in mozIServicesLogSink logger); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Register(const nsAString& target, mozIServicesLogSink *logger) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIAppServicesLogger, MOZIAPPSERVICESLOGGER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZIAPPSERVICESLOGGER \
  NS_IMETHOD Register(const nsAString& target, mozIServicesLogSink *logger) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZIAPPSERVICESLOGGER \
  nsresult Register(const nsAString& target, mozIServicesLogSink *logger); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZIAPPSERVICESLOGGER(_to) \
  NS_IMETHOD Register(const nsAString& target, mozIServicesLogSink *logger) override { return _to Register(target, logger); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZIAPPSERVICESLOGGER(_to) \
  NS_IMETHOD Register(const nsAString& target, mozIServicesLogSink *logger) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Register(target, logger); } 


#endif /* __gen_mozIAppServicesLogger_h__ */
