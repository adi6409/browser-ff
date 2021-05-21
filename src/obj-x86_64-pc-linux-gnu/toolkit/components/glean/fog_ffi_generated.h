/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/* Generated with cbindgen:0.16.0 */

/* DO NOT MODIFY THIS MANUALLY! This file was generated using cbindgen. */

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>
#include "nsTArray.h"
#include "nsString.h"

namespace mozilla::glean::impl {

extern "C" {

/// Project FOG's entry point.
///
/// This assembles client information and the Glean configuration and then initializes the global
/// Glean instance.
nsresult fog_init();

void fog_shutdown();

/// Only safe if only called on a single thread (the same single thread you call
/// fog_give_ipc_buf on).
uintptr_t fog_serialize_ipc_buf();

/// Only safe if called on a single thread (the same single thread you call
/// fog_serialize_ipc_buf on), and if buf points to an allocated buffer of at
/// least buf_len bytes.
uintptr_t fog_give_ipc_buf(uint8_t *buf, uintptr_t buf_len);

/// Only safe if buf points to an allocated buffer of at least buf_len bytes.
/// No ownership is transfered to Rust by this method: caller owns the memory at
/// buf before and after this call.
void fog_use_ipc_buf(const uint8_t *buf, uintptr_t buf_len);

/// Sets the debug tag for pings assembled in the future.
/// Returns an error result if the provided value is not a valid tag.
nsresult fog_set_debug_view_tag(const nsACString *value);

/// Submits a ping by name.
nsresult fog_submit_ping(const nsACString *ping_name);

/// Turns ping logging on or off.
/// Returns an error if the logging failed to be configured.
nsresult fog_set_log_pings(bool value);

void fog_counter_add(uint32_t id, int32_t amount);

bool fog_counter_test_has_value(uint32_t id, const nsACString *storage_name);

int32_t fog_counter_test_get_value(uint32_t id, const nsACString *storage_name);

void fog_timespan_start(uint32_t id);

void fog_timespan_stop(uint32_t id);

bool fog_timespan_test_has_value(uint32_t id, const nsACString *storage_name);

uint64_t fog_timespan_test_get_value(uint32_t id, const nsACString *storage_name);

bool fog_boolean_test_has_value(uint32_t id, const nsACString *storage_name);

bool fog_boolean_test_get_value(uint32_t id, const nsACString *storage_name);

void fog_boolean_set(uint32_t id, bool value);

bool fog_string_test_has_value(uint32_t id, const nsACString *storage_name);

void fog_string_test_get_value(uint32_t id, const nsACString *storage_name, nsACString *value);

void fog_string_set(uint32_t id, const nsACString *value);

bool fog_string_list_test_has_value(uint32_t id, const nsACString *storage_name);

void fog_string_list_test_get_value(uint32_t id,
                                    const nsACString *storage_name,
                                    nsTArray<nsCString> *value);

void fog_string_list_add(uint32_t id, const nsACString *value);

void fog_string_list_set(uint32_t id, const nsTArray<nsCString> *value);

bool fog_uuid_test_has_value(uint32_t id, const nsACString *storage_name);

void fog_uuid_test_get_value(uint32_t id, const nsACString *storage_name, nsACString *value);

void fog_uuid_set(uint32_t id, const nsACString *value);

void fog_uuid_generate_and_set(uint32_t id);

bool fog_datetime_test_has_value(uint32_t id, const nsACString *storage_name);

void fog_datetime_test_get_value(uint32_t id, const nsACString *storage_name, nsACString *value);

void fog_datetime_set(uint32_t id,
                      int32_t year,
                      uint32_t month,
                      uint32_t day,
                      uint32_t hour,
                      uint32_t minute,
                      uint32_t second,
                      uint32_t nano,
                      int32_t offset_seconds);

bool fog_memory_distribution_test_has_value(uint32_t id, const nsACString *storage_name);

void fog_memory_distribution_test_get_value(uint32_t id,
                                            const nsACString *storage_name,
                                            uint64_t *sum,
                                            nsTArray<uint64_t> *buckets,
                                            nsTArray<uint64_t> *counts);

void fog_memory_distribution_accumulate(uint32_t id, uint64_t sample);

void fog_submit_ping_by_id(uint32_t id, const nsACString *reason);

uint64_t fog_timing_distribution_start(uint32_t id);

void fog_timing_distribution_stop_and_accumulate(uint32_t id, uint64_t timing_id);

void fog_timing_distribution_cancel(uint32_t id, uint64_t timing_id);

bool fog_timing_distribution_test_has_value(uint32_t id, const nsACString *ping_name);

void fog_timing_distribution_test_get_value(uint32_t id,
                                            const nsACString *ping_name,
                                            uint64_t *sum,
                                            nsTArray<uint64_t> *buckets,
                                            nsTArray<uint64_t> *counts);

uint32_t fog_labeled_boolean_get(uint32_t id, const nsACString *label);

uint32_t fog_labeled_counter_get(uint32_t id, const nsACString *label);

uint32_t fog_labeled_string_get(uint32_t id, const nsACString *label);

void fog_event_record(uint32_t id,
                      const nsTArray<int32_t> *extra_keys,
                      const nsTArray<nsCString> *extra_values);

void fog_event_record_str(uint32_t id,
                          const nsTArray<nsCString> *extra_keys,
                          const nsTArray<nsCString> *extra_values);

bool fog_event_test_has_value(uint32_t id, const nsACString *storage_name);

} // extern "C"

} // namespace mozilla::glean::impl
