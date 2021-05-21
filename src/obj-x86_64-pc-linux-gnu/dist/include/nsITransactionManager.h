/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/txmgr/nsITransactionManager.idl
 */

#ifndef __gen_nsITransactionManager_h__
#define __gen_nsITransactionManager_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsITransaction_h__
#include "nsITransaction.h"
#endif

#ifndef __gen_nsITransactionListener_h__
#include "nsITransactionListener.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
class TransactionManager;
} // namespace mozilla

/* starting interface:    nsITransactionManager */
#define NS_ITRANSACTIONMANAGER_IID_STR "c77763df-0fb9-41a8-8074-8e882f605755"

#define NS_ITRANSACTIONMANAGER_IID \
  {0xc77763df, 0x0fb9, 0x41a8, \
    { 0x80, 0x74, 0x8e, 0x88, 0x2f, 0x60, 0x57, 0x55 }}

class nsITransactionManager : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITRANSACTIONMANAGER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITransactionManager;

  /* [can_run_script] void doTransaction (in nsITransaction aTransaction); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoTransaction(nsITransaction *aTransaction) = 0;

  /* [can_run_script] void undoTransaction (); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD UndoTransaction(void) = 0;

  /* [can_run_script] void redoTransaction (); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RedoTransaction(void) = 0;

  /* void clear (); */
  NS_IMETHOD Clear(void) = 0;

  /* void clearUndoStack (); */
  NS_IMETHOD ClearUndoStack(void) = 0;

  /* void clearRedoStack (); */
  NS_IMETHOD ClearRedoStack(void) = 0;

  /* [can_run_script] void beginBatch (in nsISupports aData); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD BeginBatch(nsISupports *aData) = 0;

  /* void endBatch (in boolean aAllowEmpty); */
  NS_IMETHOD EndBatch(bool aAllowEmpty) = 0;

  /* readonly attribute long numberOfUndoItems; */
  NS_IMETHOD GetNumberOfUndoItems(int32_t *aNumberOfUndoItems) = 0;

  /* readonly attribute long numberOfRedoItems; */
  NS_IMETHOD GetNumberOfRedoItems(int32_t *aNumberOfRedoItems) = 0;

  /* attribute long maxTransactionCount; */
  NS_IMETHOD GetMaxTransactionCount(int32_t *aMaxTransactionCount) = 0;
  NS_IMETHOD SetMaxTransactionCount(int32_t aMaxTransactionCount) = 0;

  /* void batchTopUndo (); */
  NS_IMETHOD BatchTopUndo(void) = 0;

  /* void removeTopUndo (); */
  NS_IMETHOD RemoveTopUndo(void) = 0;

  /* nsITransaction peekUndoStack (); */
  NS_IMETHOD PeekUndoStack(nsITransaction **_retval) = 0;

  /* nsITransaction peekRedoStack (); */
  NS_IMETHOD PeekRedoStack(nsITransaction **_retval) = 0;

  /* void AddListener (in nsITransactionListener aListener); */
  NS_IMETHOD AddListener(nsITransactionListener *aListener) = 0;

  /* void RemoveListener (in nsITransactionListener aListener); */
  NS_IMETHOD RemoveListener(nsITransactionListener *aListener) = 0;

   /**
   * AsTransactionManager() returns a pointer to TransactionManager class.
   *
   * In order to avoid circular dependency issues, this method is defined
   * in mozilla/TransactionManager.h.  Consumers need to #include that header.
   */
  inline mozilla::TransactionManager* AsTransactionManager();
};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITransactionManager, NS_ITRANSACTIONMANAGER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITRANSACTIONMANAGER \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoTransaction(nsITransaction *aTransaction) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD UndoTransaction(void) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RedoTransaction(void) override; \
  NS_IMETHOD Clear(void) override; \
  NS_IMETHOD ClearUndoStack(void) override; \
  NS_IMETHOD ClearRedoStack(void) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD BeginBatch(nsISupports *aData) override; \
  NS_IMETHOD EndBatch(bool aAllowEmpty) override; \
  NS_IMETHOD GetNumberOfUndoItems(int32_t *aNumberOfUndoItems) override; \
  NS_IMETHOD GetNumberOfRedoItems(int32_t *aNumberOfRedoItems) override; \
  NS_IMETHOD GetMaxTransactionCount(int32_t *aMaxTransactionCount) override; \
  NS_IMETHOD SetMaxTransactionCount(int32_t aMaxTransactionCount) override; \
  NS_IMETHOD BatchTopUndo(void) override; \
  NS_IMETHOD RemoveTopUndo(void) override; \
  NS_IMETHOD PeekUndoStack(nsITransaction **_retval) override; \
  NS_IMETHOD PeekRedoStack(nsITransaction **_retval) override; \
  NS_IMETHOD AddListener(nsITransactionListener *aListener) override; \
  NS_IMETHOD RemoveListener(nsITransactionListener *aListener) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITRANSACTIONMANAGER \
  MOZ_CAN_RUN_SCRIPT nsresult DoTransaction(nsITransaction *aTransaction); \
  MOZ_CAN_RUN_SCRIPT nsresult UndoTransaction(void); \
  MOZ_CAN_RUN_SCRIPT nsresult RedoTransaction(void); \
  nsresult Clear(void); \
  nsresult ClearUndoStack(void); \
  nsresult ClearRedoStack(void); \
  MOZ_CAN_RUN_SCRIPT nsresult BeginBatch(nsISupports *aData); \
  nsresult EndBatch(bool aAllowEmpty); \
  nsresult GetNumberOfUndoItems(int32_t *aNumberOfUndoItems); \
  nsresult GetNumberOfRedoItems(int32_t *aNumberOfRedoItems); \
  nsresult GetMaxTransactionCount(int32_t *aMaxTransactionCount); \
  nsresult SetMaxTransactionCount(int32_t aMaxTransactionCount); \
  nsresult BatchTopUndo(void); \
  nsresult RemoveTopUndo(void); \
  nsresult PeekUndoStack(nsITransaction **_retval); \
  nsresult PeekRedoStack(nsITransaction **_retval); \
  nsresult AddListener(nsITransactionListener *aListener); \
  nsresult RemoveListener(nsITransactionListener *aListener); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITRANSACTIONMANAGER(_to) \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoTransaction(nsITransaction *aTransaction) override { return _to DoTransaction(aTransaction); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD UndoTransaction(void) override { return _to UndoTransaction(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RedoTransaction(void) override { return _to RedoTransaction(); } \
  NS_IMETHOD Clear(void) override { return _to Clear(); } \
  NS_IMETHOD ClearUndoStack(void) override { return _to ClearUndoStack(); } \
  NS_IMETHOD ClearRedoStack(void) override { return _to ClearRedoStack(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD BeginBatch(nsISupports *aData) override { return _to BeginBatch(aData); } \
  NS_IMETHOD EndBatch(bool aAllowEmpty) override { return _to EndBatch(aAllowEmpty); } \
  NS_IMETHOD GetNumberOfUndoItems(int32_t *aNumberOfUndoItems) override { return _to GetNumberOfUndoItems(aNumberOfUndoItems); } \
  NS_IMETHOD GetNumberOfRedoItems(int32_t *aNumberOfRedoItems) override { return _to GetNumberOfRedoItems(aNumberOfRedoItems); } \
  NS_IMETHOD GetMaxTransactionCount(int32_t *aMaxTransactionCount) override { return _to GetMaxTransactionCount(aMaxTransactionCount); } \
  NS_IMETHOD SetMaxTransactionCount(int32_t aMaxTransactionCount) override { return _to SetMaxTransactionCount(aMaxTransactionCount); } \
  NS_IMETHOD BatchTopUndo(void) override { return _to BatchTopUndo(); } \
  NS_IMETHOD RemoveTopUndo(void) override { return _to RemoveTopUndo(); } \
  NS_IMETHOD PeekUndoStack(nsITransaction **_retval) override { return _to PeekUndoStack(_retval); } \
  NS_IMETHOD PeekRedoStack(nsITransaction **_retval) override { return _to PeekRedoStack(_retval); } \
  NS_IMETHOD AddListener(nsITransactionListener *aListener) override { return _to AddListener(aListener); } \
  NS_IMETHOD RemoveListener(nsITransactionListener *aListener) override { return _to RemoveListener(aListener); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITRANSACTIONMANAGER(_to) \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoTransaction(nsITransaction *aTransaction) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DoTransaction(aTransaction); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD UndoTransaction(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UndoTransaction(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RedoTransaction(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RedoTransaction(); } \
  NS_IMETHOD Clear(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Clear(); } \
  NS_IMETHOD ClearUndoStack(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearUndoStack(); } \
  NS_IMETHOD ClearRedoStack(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearRedoStack(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD BeginBatch(nsISupports *aData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->BeginBatch(aData); } \
  NS_IMETHOD EndBatch(bool aAllowEmpty) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EndBatch(aAllowEmpty); } \
  NS_IMETHOD GetNumberOfUndoItems(int32_t *aNumberOfUndoItems) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNumberOfUndoItems(aNumberOfUndoItems); } \
  NS_IMETHOD GetNumberOfRedoItems(int32_t *aNumberOfRedoItems) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNumberOfRedoItems(aNumberOfRedoItems); } \
  NS_IMETHOD GetMaxTransactionCount(int32_t *aMaxTransactionCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMaxTransactionCount(aMaxTransactionCount); } \
  NS_IMETHOD SetMaxTransactionCount(int32_t aMaxTransactionCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMaxTransactionCount(aMaxTransactionCount); } \
  NS_IMETHOD BatchTopUndo(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->BatchTopUndo(); } \
  NS_IMETHOD RemoveTopUndo(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveTopUndo(); } \
  NS_IMETHOD PeekUndoStack(nsITransaction **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PeekUndoStack(_retval); } \
  NS_IMETHOD PeekRedoStack(nsITransaction **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PeekRedoStack(_retval); } \
  NS_IMETHOD AddListener(nsITransactionListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddListener(aListener); } \
  NS_IMETHOD RemoveListener(nsITransactionListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveListener(aListener); } \


#endif /* __gen_nsITransactionManager_h__ */
