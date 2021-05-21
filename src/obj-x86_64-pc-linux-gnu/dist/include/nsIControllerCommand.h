/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/commandhandler/nsIControllerCommand.idl
 */

#ifndef __gen_nsIControllerCommand_h__
#define __gen_nsIControllerCommand_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsICommandParams_h__
#include "nsICommandParams.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIControllerCommand */
#define NS_ICONTROLLERCOMMAND_IID_STR "0eae9a46-1dd2-11b2-aca0-9176f05fe9db"

#define NS_ICONTROLLERCOMMAND_IID \
  {0x0eae9a46, 0x1dd2, 0x11b2, \
    { 0xac, 0xa0, 0x91, 0x76, 0xf0, 0x5f, 0xe9, 0xdb }}

class NS_NO_VTABLE nsIControllerCommand : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICONTROLLERCOMMAND_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIControllerCommand;

  /* boolean isCommandEnabled (in string aCommandName, in nsISupports aCommandContext); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsCommandEnabled(const char * aCommandName, nsISupports *aCommandContext, bool *_retval) = 0;

  /* void getCommandStateParams (in string aCommandName, in nsICommandParams aParams, in nsISupports aCommandContext); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCommandStateParams(const char * aCommandName, nsICommandParams *aParams, nsISupports *aCommandContext) = 0;

  /* [can_run_script] void doCommand (in string aCommandName, in nsISupports aCommandContext); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommand(const char * aCommandName, nsISupports *aCommandContext) = 0;

  /* [can_run_script] void doCommandParams (in string aCommandName, in nsICommandParams aParams, in nsISupports aCommandContext); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommandParams(const char * aCommandName, nsICommandParams *aParams, nsISupports *aCommandContext) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIControllerCommand, NS_ICONTROLLERCOMMAND_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICONTROLLERCOMMAND \
  NS_IMETHOD IsCommandEnabled(const char * aCommandName, nsISupports *aCommandContext, bool *_retval) override; \
  NS_IMETHOD GetCommandStateParams(const char * aCommandName, nsICommandParams *aParams, nsISupports *aCommandContext) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommand(const char * aCommandName, nsISupports *aCommandContext) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommandParams(const char * aCommandName, nsICommandParams *aParams, nsISupports *aCommandContext) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICONTROLLERCOMMAND \
  nsresult IsCommandEnabled(const char * aCommandName, nsISupports *aCommandContext, bool *_retval); \
  nsresult GetCommandStateParams(const char * aCommandName, nsICommandParams *aParams, nsISupports *aCommandContext); \
  MOZ_CAN_RUN_SCRIPT nsresult DoCommand(const char * aCommandName, nsISupports *aCommandContext); \
  MOZ_CAN_RUN_SCRIPT nsresult DoCommandParams(const char * aCommandName, nsICommandParams *aParams, nsISupports *aCommandContext); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICONTROLLERCOMMAND(_to) \
  NS_IMETHOD IsCommandEnabled(const char * aCommandName, nsISupports *aCommandContext, bool *_retval) override { return _to IsCommandEnabled(aCommandName, aCommandContext, _retval); } \
  NS_IMETHOD GetCommandStateParams(const char * aCommandName, nsICommandParams *aParams, nsISupports *aCommandContext) override { return _to GetCommandStateParams(aCommandName, aParams, aCommandContext); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommand(const char * aCommandName, nsISupports *aCommandContext) override { return _to DoCommand(aCommandName, aCommandContext); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommandParams(const char * aCommandName, nsICommandParams *aParams, nsISupports *aCommandContext) override { return _to DoCommandParams(aCommandName, aParams, aCommandContext); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICONTROLLERCOMMAND(_to) \
  NS_IMETHOD IsCommandEnabled(const char * aCommandName, nsISupports *aCommandContext, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsCommandEnabled(aCommandName, aCommandContext, _retval); } \
  NS_IMETHOD GetCommandStateParams(const char * aCommandName, nsICommandParams *aParams, nsISupports *aCommandContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCommandStateParams(aCommandName, aParams, aCommandContext); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommand(const char * aCommandName, nsISupports *aCommandContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DoCommand(aCommandName, aCommandContext); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommandParams(const char * aCommandName, nsICommandParams *aParams, nsISupports *aCommandContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DoCommandParams(aCommandName, aParams, aCommandContext); } 


#endif /* __gen_nsIControllerCommand_h__ */
