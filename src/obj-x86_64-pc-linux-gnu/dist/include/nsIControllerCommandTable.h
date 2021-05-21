/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/commandhandler/nsIControllerCommandTable.idl
 */

#ifndef __gen_nsIControllerCommandTable_h__
#define __gen_nsIControllerCommandTable_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIControllerCommand_h__
#include "nsIControllerCommand.h"
#endif

#ifndef __gen_nsICommandParams_h__
#include "nsICommandParams.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsControllerCommandTable;

/* starting interface:    nsIControllerCommandTable */
#define NS_ICONTROLLERCOMMANDTABLE_IID_STR "c847f90e-b8f3-49db-a4df-8867831f2800"

#define NS_ICONTROLLERCOMMANDTABLE_IID \
  {0xc847f90e, 0xb8f3, 0x49db, \
    { 0xa4, 0xdf, 0x88, 0x67, 0x83, 0x1f, 0x28, 0x00 }}

class nsIControllerCommandTable : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICONTROLLERCOMMANDTABLE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIControllerCommandTable;

  /* void makeImmutable (); */
  NS_IMETHOD MakeImmutable(void) = 0;

  /* void registerCommand (in string aCommandName, in nsIControllerCommand aCommand); */
  NS_IMETHOD RegisterCommand(const char * aCommandName, nsIControllerCommand *aCommand) = 0;

  /* void unregisterCommand (in string aCommandName, in nsIControllerCommand aCommand); */
  NS_IMETHOD UnregisterCommand(const char * aCommandName, nsIControllerCommand *aCommand) = 0;

  /* nsIControllerCommand findCommandHandler (in string aCommandName); */
  NS_IMETHOD FindCommandHandler(const char * aCommandName, nsIControllerCommand **_retval) = 0;

  /* boolean isCommandEnabled (in string aCommandName, in nsISupports aCommandRefCon); */
  NS_IMETHOD IsCommandEnabled(const char * aCommandName, nsISupports *aCommandRefCon, bool *_retval) = 0;

  /* void updateCommandState (in string aCommandName, in nsISupports aCommandRefCon); */
  NS_IMETHOD UpdateCommandState(const char * aCommandName, nsISupports *aCommandRefCon) = 0;

  /* boolean supportsCommand (in string aCommandName, in nsISupports aCommandRefCon); */
  NS_IMETHOD SupportsCommand(const char * aCommandName, nsISupports *aCommandRefCon, bool *_retval) = 0;

  /* [can_run_script] void doCommand (in string aCommandName, in nsISupports aCommandRefCon); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommand(const char * aCommandName, nsISupports *aCommandRefCon) = 0;

  /* [can_run_script] void doCommandParams (in string aCommandName, in nsICommandParams aParam, in nsISupports aCommandRefCon); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommandParams(const char * aCommandName, nsICommandParams *aParam, nsISupports *aCommandRefCon) = 0;

  /* void getCommandState (in string aCommandName, in nsICommandParams aParam, in nsISupports aCommandRefCon); */
  NS_IMETHOD GetCommandState(const char * aCommandName, nsICommandParams *aParam, nsISupports *aCommandRefCon) = 0;

  /* Array<ACString> getSupportedCommands (); */
  NS_IMETHOD GetSupportedCommands(nsTArray<nsCString >& _retval) = 0;

   /**
   * In order to avoid circular dependency issues, these methods are defined
   * in nsControllerCommandTable.h.  Consumers need to #include that header.
   */
  inline nsControllerCommandTable* AsControllerCommandTable();
  inline const nsControllerCommandTable* AsControllerCommandTable() const;
};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIControllerCommandTable, NS_ICONTROLLERCOMMANDTABLE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICONTROLLERCOMMANDTABLE \
  NS_IMETHOD MakeImmutable(void) override; \
  NS_IMETHOD RegisterCommand(const char * aCommandName, nsIControllerCommand *aCommand) override; \
  NS_IMETHOD UnregisterCommand(const char * aCommandName, nsIControllerCommand *aCommand) override; \
  NS_IMETHOD FindCommandHandler(const char * aCommandName, nsIControllerCommand **_retval) override; \
  NS_IMETHOD IsCommandEnabled(const char * aCommandName, nsISupports *aCommandRefCon, bool *_retval) override; \
  NS_IMETHOD UpdateCommandState(const char * aCommandName, nsISupports *aCommandRefCon) override; \
  NS_IMETHOD SupportsCommand(const char * aCommandName, nsISupports *aCommandRefCon, bool *_retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommand(const char * aCommandName, nsISupports *aCommandRefCon) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommandParams(const char * aCommandName, nsICommandParams *aParam, nsISupports *aCommandRefCon) override; \
  NS_IMETHOD GetCommandState(const char * aCommandName, nsICommandParams *aParam, nsISupports *aCommandRefCon) override; \
  NS_IMETHOD GetSupportedCommands(nsTArray<nsCString >& _retval) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICONTROLLERCOMMANDTABLE \
  nsresult MakeImmutable(void); \
  nsresult RegisterCommand(const char * aCommandName, nsIControllerCommand *aCommand); \
  nsresult UnregisterCommand(const char * aCommandName, nsIControllerCommand *aCommand); \
  nsresult FindCommandHandler(const char * aCommandName, nsIControllerCommand **_retval); \
  nsresult IsCommandEnabled(const char * aCommandName, nsISupports *aCommandRefCon, bool *_retval); \
  nsresult UpdateCommandState(const char * aCommandName, nsISupports *aCommandRefCon); \
  nsresult SupportsCommand(const char * aCommandName, nsISupports *aCommandRefCon, bool *_retval); \
  MOZ_CAN_RUN_SCRIPT nsresult DoCommand(const char * aCommandName, nsISupports *aCommandRefCon); \
  MOZ_CAN_RUN_SCRIPT nsresult DoCommandParams(const char * aCommandName, nsICommandParams *aParam, nsISupports *aCommandRefCon); \
  nsresult GetCommandState(const char * aCommandName, nsICommandParams *aParam, nsISupports *aCommandRefCon); \
  nsresult GetSupportedCommands(nsTArray<nsCString >& _retval); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICONTROLLERCOMMANDTABLE(_to) \
  NS_IMETHOD MakeImmutable(void) override { return _to MakeImmutable(); } \
  NS_IMETHOD RegisterCommand(const char * aCommandName, nsIControllerCommand *aCommand) override { return _to RegisterCommand(aCommandName, aCommand); } \
  NS_IMETHOD UnregisterCommand(const char * aCommandName, nsIControllerCommand *aCommand) override { return _to UnregisterCommand(aCommandName, aCommand); } \
  NS_IMETHOD FindCommandHandler(const char * aCommandName, nsIControllerCommand **_retval) override { return _to FindCommandHandler(aCommandName, _retval); } \
  NS_IMETHOD IsCommandEnabled(const char * aCommandName, nsISupports *aCommandRefCon, bool *_retval) override { return _to IsCommandEnabled(aCommandName, aCommandRefCon, _retval); } \
  NS_IMETHOD UpdateCommandState(const char * aCommandName, nsISupports *aCommandRefCon) override { return _to UpdateCommandState(aCommandName, aCommandRefCon); } \
  NS_IMETHOD SupportsCommand(const char * aCommandName, nsISupports *aCommandRefCon, bool *_retval) override { return _to SupportsCommand(aCommandName, aCommandRefCon, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommand(const char * aCommandName, nsISupports *aCommandRefCon) override { return _to DoCommand(aCommandName, aCommandRefCon); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommandParams(const char * aCommandName, nsICommandParams *aParam, nsISupports *aCommandRefCon) override { return _to DoCommandParams(aCommandName, aParam, aCommandRefCon); } \
  NS_IMETHOD GetCommandState(const char * aCommandName, nsICommandParams *aParam, nsISupports *aCommandRefCon) override { return _to GetCommandState(aCommandName, aParam, aCommandRefCon); } \
  NS_IMETHOD GetSupportedCommands(nsTArray<nsCString >& _retval) override { return _to GetSupportedCommands(_retval); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICONTROLLERCOMMANDTABLE(_to) \
  NS_IMETHOD MakeImmutable(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MakeImmutable(); } \
  NS_IMETHOD RegisterCommand(const char * aCommandName, nsIControllerCommand *aCommand) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterCommand(aCommandName, aCommand); } \
  NS_IMETHOD UnregisterCommand(const char * aCommandName, nsIControllerCommand *aCommand) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnregisterCommand(aCommandName, aCommand); } \
  NS_IMETHOD FindCommandHandler(const char * aCommandName, nsIControllerCommand **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FindCommandHandler(aCommandName, _retval); } \
  NS_IMETHOD IsCommandEnabled(const char * aCommandName, nsISupports *aCommandRefCon, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsCommandEnabled(aCommandName, aCommandRefCon, _retval); } \
  NS_IMETHOD UpdateCommandState(const char * aCommandName, nsISupports *aCommandRefCon) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UpdateCommandState(aCommandName, aCommandRefCon); } \
  NS_IMETHOD SupportsCommand(const char * aCommandName, nsISupports *aCommandRefCon, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SupportsCommand(aCommandName, aCommandRefCon, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommand(const char * aCommandName, nsISupports *aCommandRefCon) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DoCommand(aCommandName, aCommandRefCon); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommandParams(const char * aCommandName, nsICommandParams *aParam, nsISupports *aCommandRefCon) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DoCommandParams(aCommandName, aParam, aCommandRefCon); } \
  NS_IMETHOD GetCommandState(const char * aCommandName, nsICommandParams *aParam, nsISupports *aCommandRefCon) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCommandState(aCommandName, aParam, aCommandRefCon); } \
  NS_IMETHOD GetSupportedCommands(nsTArray<nsCString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSupportedCommands(_retval); } \


#endif /* __gen_nsIControllerCommandTable_h__ */
