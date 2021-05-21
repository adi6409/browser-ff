/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/txmgr/nsITransaction.idl
 */

#ifndef __gen_nsITransaction_h__
#define __gen_nsITransaction_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
class EditTransactionBase;
}
template <typename T>
struct already_AddRefed;

/* starting interface:    nsITransaction */
#define NS_ITRANSACTION_IID_STR "58e330c1-7b48-11d2-98b9-00805f297d89"

#define NS_ITRANSACTION_IID \
  {0x58e330c1, 0x7b48, 0x11d2, \
    { 0x98, 0xb9, 0x00, 0x80, 0x5f, 0x29, 0x7d, 0x89 }}

class nsITransaction : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITRANSACTION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITransaction;

  /* [can_run_script] void doTransaction (); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoTransaction(void) = 0;

  /* [can_run_script] void undoTransaction (); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD UndoTransaction(void) = 0;

  /* [can_run_script] void redoTransaction (); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD RedoTransaction(void) = 0;

  /* readonly attribute boolean isTransient; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsTransient(bool *aIsTransient) = 0;

  /* boolean merge (in nsITransaction aTransaction); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Merge(nsITransaction *aTransaction, bool *_retval) = 0;

  /* [noscript] EditTransactionBasePtr getAsEditTransactionBase (); */
  NS_IMETHOD GetAsEditTransactionBase(mozilla::EditTransactionBase * * _retval) = 0;

   inline already_AddRefed<mozilla::EditTransactionBase>
  GetAsEditTransactionBase();
};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITransaction, NS_ITRANSACTION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITRANSACTION \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoTransaction(void) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD UndoTransaction(void) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RedoTransaction(void) override; \
  NS_IMETHOD GetIsTransient(bool *aIsTransient) override; \
  NS_IMETHOD Merge(nsITransaction *aTransaction, bool *_retval) override; \
  NS_IMETHOD GetAsEditTransactionBase(mozilla::EditTransactionBase * * _retval) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITRANSACTION \
  MOZ_CAN_RUN_SCRIPT nsresult DoTransaction(void); \
  MOZ_CAN_RUN_SCRIPT nsresult UndoTransaction(void); \
  MOZ_CAN_RUN_SCRIPT nsresult RedoTransaction(void); \
  nsresult GetIsTransient(bool *aIsTransient); \
  nsresult Merge(nsITransaction *aTransaction, bool *_retval); \
  nsresult GetAsEditTransactionBase(mozilla::EditTransactionBase * * _retval); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITRANSACTION(_to) \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoTransaction(void) override { return _to DoTransaction(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD UndoTransaction(void) override { return _to UndoTransaction(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RedoTransaction(void) override { return _to RedoTransaction(); } \
  NS_IMETHOD GetIsTransient(bool *aIsTransient) override { return _to GetIsTransient(aIsTransient); } \
  NS_IMETHOD Merge(nsITransaction *aTransaction, bool *_retval) override { return _to Merge(aTransaction, _retval); } \
  NS_IMETHOD GetAsEditTransactionBase(mozilla::EditTransactionBase * * _retval) override { return _to GetAsEditTransactionBase(_retval); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITRANSACTION(_to) \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoTransaction(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DoTransaction(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD UndoTransaction(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UndoTransaction(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RedoTransaction(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RedoTransaction(); } \
  NS_IMETHOD GetIsTransient(bool *aIsTransient) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsTransient(aIsTransient); } \
  NS_IMETHOD Merge(nsITransaction *aTransaction, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Merge(aTransaction, _retval); } \
  NS_IMETHOD GetAsEditTransactionBase(mozilla::EditTransactionBase * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsEditTransactionBase(_retval); } \


#endif /* __gen_nsITransaction_h__ */
