/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/xul/nsIController.idl
 */

#ifndef __gen_nsIController_h__
#define __gen_nsIController_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIController */
#define NS_ICONTROLLER_IID_STR "d5b61b82-1da4-11d3-bf87-00105a1b0627"

#define NS_ICONTROLLER_IID \
  {0xd5b61b82, 0x1da4, 0x11d3, \
    { 0xbf, 0x87, 0x00, 0x10, 0x5a, 0x1b, 0x06, 0x27 }}

class NS_NO_VTABLE nsIController : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICONTROLLER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIController;

  /* boolean isCommandEnabled (in string command); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsCommandEnabled(const char * command, bool *_retval) = 0;

  /* boolean supportsCommand (in string command); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SupportsCommand(const char * command, bool *_retval) = 0;

  /* [can_run_script] void doCommand (in string command); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommand(const char * command) = 0;

  /* void onEvent (in string eventName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnEvent(const char * eventName) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIController, NS_ICONTROLLER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICONTROLLER \
  NS_IMETHOD IsCommandEnabled(const char * command, bool *_retval) override; \
  NS_IMETHOD SupportsCommand(const char * command, bool *_retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommand(const char * command) override; \
  NS_IMETHOD OnEvent(const char * eventName) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICONTROLLER \
  nsresult IsCommandEnabled(const char * command, bool *_retval); \
  nsresult SupportsCommand(const char * command, bool *_retval); \
  MOZ_CAN_RUN_SCRIPT nsresult DoCommand(const char * command); \
  nsresult OnEvent(const char * eventName); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICONTROLLER(_to) \
  NS_IMETHOD IsCommandEnabled(const char * command, bool *_retval) override { return _to IsCommandEnabled(command, _retval); } \
  NS_IMETHOD SupportsCommand(const char * command, bool *_retval) override { return _to SupportsCommand(command, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommand(const char * command) override { return _to DoCommand(command); } \
  NS_IMETHOD OnEvent(const char * eventName) override { return _to OnEvent(eventName); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICONTROLLER(_to) \
  NS_IMETHOD IsCommandEnabled(const char * command, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsCommandEnabled(command, _retval); } \
  NS_IMETHOD SupportsCommand(const char * command, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SupportsCommand(command, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommand(const char * command) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DoCommand(command); } \
  NS_IMETHOD OnEvent(const char * eventName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnEvent(eventName); } 

class nsICommandParams; /* forward declaration */


/* starting interface:    nsICommandController */
#define NS_ICOMMANDCONTROLLER_IID_STR "eec0b435-7f53-44fe-b00a-cf3eed65c01a"

#define NS_ICOMMANDCONTROLLER_IID \
  {0xeec0b435, 0x7f53, 0x44fe, \
    { 0xb0, 0x0a, 0xcf, 0x3e, 0xed, 0x65, 0xc0, 0x1a }}

class NS_NO_VTABLE nsICommandController : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICOMMANDCONTROLLER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICommandController;

  /* void getCommandStateWithParams (in string command, in nsICommandParams aCommandParams); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCommandStateWithParams(const char * command, nsICommandParams *aCommandParams) = 0;

  /* [can_run_script] void doCommandWithParams (in string command, in nsICommandParams aCommandParams); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommandWithParams(const char * command, nsICommandParams *aCommandParams) = 0;

  /* Array<ACString> getSupportedCommands (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSupportedCommands(nsTArray<nsCString >& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICommandController, NS_ICOMMANDCONTROLLER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICOMMANDCONTROLLER \
  NS_IMETHOD GetCommandStateWithParams(const char * command, nsICommandParams *aCommandParams) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommandWithParams(const char * command, nsICommandParams *aCommandParams) override; \
  NS_IMETHOD GetSupportedCommands(nsTArray<nsCString >& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICOMMANDCONTROLLER \
  nsresult GetCommandStateWithParams(const char * command, nsICommandParams *aCommandParams); \
  MOZ_CAN_RUN_SCRIPT nsresult DoCommandWithParams(const char * command, nsICommandParams *aCommandParams); \
  nsresult GetSupportedCommands(nsTArray<nsCString >& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICOMMANDCONTROLLER(_to) \
  NS_IMETHOD GetCommandStateWithParams(const char * command, nsICommandParams *aCommandParams) override { return _to GetCommandStateWithParams(command, aCommandParams); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommandWithParams(const char * command, nsICommandParams *aCommandParams) override { return _to DoCommandWithParams(command, aCommandParams); } \
  NS_IMETHOD GetSupportedCommands(nsTArray<nsCString >& _retval) override { return _to GetSupportedCommands(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICOMMANDCONTROLLER(_to) \
  NS_IMETHOD GetCommandStateWithParams(const char * command, nsICommandParams *aCommandParams) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCommandStateWithParams(command, aCommandParams); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommandWithParams(const char * command, nsICommandParams *aCommandParams) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DoCommandWithParams(command, aCommandParams); } \
  NS_IMETHOD GetSupportedCommands(nsTArray<nsCString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSupportedCommands(_retval); } 


#endif /* __gen_nsIController_h__ */
