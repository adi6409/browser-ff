/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/commandhandler/nsICommandManager.idl
 */

#ifndef __gen_nsICommandManager_h__
#define __gen_nsICommandManager_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIObserver_h__
#include "nsIObserver.h"
#endif

#ifndef __gen_nsICommandParams_h__
#include "nsICommandParams.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIDOMWindowProxy; /* forward declaration */

class nsCommandManager;

/* starting interface:    nsICommandManager */
#define NS_ICOMMANDMANAGER_IID_STR "bb5a1730-d83b-4fa2-831b-35b9d5842e84"

#define NS_ICOMMANDMANAGER_IID \
  {0xbb5a1730, 0xd83b, 0x4fa2, \
    { 0x83, 0x1b, 0x35, 0xb9, 0xd5, 0x84, 0x2e, 0x84 }}

class nsICommandManager : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICOMMANDMANAGER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICommandManager;

  /* void addCommandObserver (in nsIObserver aCommandObserver, in string aCommandToObserve); */
  NS_IMETHOD AddCommandObserver(nsIObserver *aCommandObserver, const char * aCommandToObserve) = 0;

  /* void removeCommandObserver (in nsIObserver aCommandObserver, in string aCommandObserved); */
  NS_IMETHOD RemoveCommandObserver(nsIObserver *aCommandObserver, const char * aCommandObserved) = 0;

  /* boolean isCommandSupported (in string aCommandName, in mozIDOMWindowProxy aTargetWindow); */
  NS_IMETHOD IsCommandSupported(const char * aCommandName, mozIDOMWindowProxy *aTargetWindow, bool *_retval) = 0;

  /* boolean isCommandEnabled (in string aCommandName, in mozIDOMWindowProxy aTargetWindow); */
  NS_IMETHOD IsCommandEnabled(const char * aCommandName, mozIDOMWindowProxy *aTargetWindow, bool *_retval) = 0;

  /* void getCommandState (in string aCommandName, in mozIDOMWindowProxy aTargetWindow, in nsICommandParams aCommandParams); */
  NS_IMETHOD GetCommandState(const char * aCommandName, mozIDOMWindowProxy *aTargetWindow, nsICommandParams *aCommandParams) = 0;

  /* [can_run_script] void doCommand (in string aCommandName, in nsICommandParams aCommandParams, in mozIDOMWindowProxy aTargetWindow); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommand(const char * aCommandName, nsICommandParams *aCommandParams, mozIDOMWindowProxy *aTargetWindow) = 0;

   /**
   * In order to avoid circular dependency issues, these methods are defined
   * in nsCommandManager.h.  Consumers need to #include that header.
   */
  inline nsCommandManager* AsCommandManager();
  inline const nsCommandManager* AsCommandManager() const;
};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICommandManager, NS_ICOMMANDMANAGER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICOMMANDMANAGER \
  NS_IMETHOD AddCommandObserver(nsIObserver *aCommandObserver, const char * aCommandToObserve) override; \
  NS_IMETHOD RemoveCommandObserver(nsIObserver *aCommandObserver, const char * aCommandObserved) override; \
  NS_IMETHOD IsCommandSupported(const char * aCommandName, mozIDOMWindowProxy *aTargetWindow, bool *_retval) override; \
  NS_IMETHOD IsCommandEnabled(const char * aCommandName, mozIDOMWindowProxy *aTargetWindow, bool *_retval) override; \
  NS_IMETHOD GetCommandState(const char * aCommandName, mozIDOMWindowProxy *aTargetWindow, nsICommandParams *aCommandParams) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommand(const char * aCommandName, nsICommandParams *aCommandParams, mozIDOMWindowProxy *aTargetWindow) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICOMMANDMANAGER \
  nsresult AddCommandObserver(nsIObserver *aCommandObserver, const char * aCommandToObserve); \
  nsresult RemoveCommandObserver(nsIObserver *aCommandObserver, const char * aCommandObserved); \
  nsresult IsCommandSupported(const char * aCommandName, mozIDOMWindowProxy *aTargetWindow, bool *_retval); \
  nsresult IsCommandEnabled(const char * aCommandName, mozIDOMWindowProxy *aTargetWindow, bool *_retval); \
  nsresult GetCommandState(const char * aCommandName, mozIDOMWindowProxy *aTargetWindow, nsICommandParams *aCommandParams); \
  MOZ_CAN_RUN_SCRIPT nsresult DoCommand(const char * aCommandName, nsICommandParams *aCommandParams, mozIDOMWindowProxy *aTargetWindow); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICOMMANDMANAGER(_to) \
  NS_IMETHOD AddCommandObserver(nsIObserver *aCommandObserver, const char * aCommandToObserve) override { return _to AddCommandObserver(aCommandObserver, aCommandToObserve); } \
  NS_IMETHOD RemoveCommandObserver(nsIObserver *aCommandObserver, const char * aCommandObserved) override { return _to RemoveCommandObserver(aCommandObserver, aCommandObserved); } \
  NS_IMETHOD IsCommandSupported(const char * aCommandName, mozIDOMWindowProxy *aTargetWindow, bool *_retval) override { return _to IsCommandSupported(aCommandName, aTargetWindow, _retval); } \
  NS_IMETHOD IsCommandEnabled(const char * aCommandName, mozIDOMWindowProxy *aTargetWindow, bool *_retval) override { return _to IsCommandEnabled(aCommandName, aTargetWindow, _retval); } \
  NS_IMETHOD GetCommandState(const char * aCommandName, mozIDOMWindowProxy *aTargetWindow, nsICommandParams *aCommandParams) override { return _to GetCommandState(aCommandName, aTargetWindow, aCommandParams); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommand(const char * aCommandName, nsICommandParams *aCommandParams, mozIDOMWindowProxy *aTargetWindow) override { return _to DoCommand(aCommandName, aCommandParams, aTargetWindow); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICOMMANDMANAGER(_to) \
  NS_IMETHOD AddCommandObserver(nsIObserver *aCommandObserver, const char * aCommandToObserve) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddCommandObserver(aCommandObserver, aCommandToObserve); } \
  NS_IMETHOD RemoveCommandObserver(nsIObserver *aCommandObserver, const char * aCommandObserved) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveCommandObserver(aCommandObserver, aCommandObserved); } \
  NS_IMETHOD IsCommandSupported(const char * aCommandName, mozIDOMWindowProxy *aTargetWindow, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsCommandSupported(aCommandName, aTargetWindow, _retval); } \
  NS_IMETHOD IsCommandEnabled(const char * aCommandName, mozIDOMWindowProxy *aTargetWindow, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsCommandEnabled(aCommandName, aTargetWindow, _retval); } \
  NS_IMETHOD GetCommandState(const char * aCommandName, mozIDOMWindowProxy *aTargetWindow, nsICommandParams *aCommandParams) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCommandState(aCommandName, aTargetWindow, aCommandParams); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommand(const char * aCommandName, nsICommandParams *aCommandParams, mozIDOMWindowProxy *aTargetWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DoCommand(aCommandName, aCommandParams, aTargetWindow); } \


#endif /* __gen_nsICommandManager_h__ */
