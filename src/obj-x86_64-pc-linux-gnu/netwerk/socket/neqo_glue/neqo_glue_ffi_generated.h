/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/* Generated with cbindgen:0.16.0 */

/* DO NOT MODIFY THIS MANUALLY! This file was generated using cbindgen.
 */

namespace mozilla {
namespace net {
class NeqoHttp3Conn;
}  // namespace net
}  // namespace mozilla
 

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>
#include "certt.h"
#include "prerror.h"

namespace mozilla {
namespace net {

struct CloseError {
  enum class Tag {
    TransportInternalError,
    TransportInternalErrorOther,
    TransportError,
    CryptoError,
    CryptoAlert,
    PeerAppError,
    PeerError,
    AppError,
  };

  struct TransportInternalError_Body {
    uint16_t _0;
  };

  struct TransportInternalErrorOther_Body {
    uint16_t _0;
  };

  struct TransportError_Body {
    uint64_t _0;
  };

  struct CryptoError_Body {
    uint64_t _0;
  };

  struct CryptoAlert_Body {
    uint8_t _0;
  };

  struct PeerAppError_Body {
    uint64_t _0;
  };

  struct PeerError_Body {
    uint64_t _0;
  };

  struct AppError_Body {
    uint64_t _0;
  };

  Tag tag;
  union {
    TransportInternalError_Body transport_internal_error;
    TransportInternalErrorOther_Body transport_internal_error_other;
    TransportError_Body transport_error;
    CryptoError_Body crypto_error;
    CryptoAlert_Body crypto_alert;
    PeerAppError_Body peer_app_error;
    PeerError_Body peer_error;
    AppError_Body app_error;
  };
};

struct Http3Event {
  enum class Tag {
    /// A request stream has space for more data to be send.
    DataWritable,
    /// A server has send STOP_SENDING frame.
    StopSending,
    HeaderReady,
    /// New bytes available for reading.
    DataReadable,
    /// Peer reset the stream.
    Reset,
    /// A PushPromise
    PushPromise,
    /// A push response headers are ready.
    PushHeaderReady,
    /// New bytes are available on a push stream for reading.
    PushDataReadable,
    /// A push has been canceled.
    PushCanceled,
    PushReset,
    RequestsCreatable,
    AuthenticationNeeded,
    ZeroRttRejected,
    ConnectionConnected,
    GoawayReceived,
    ConnectionClosing,
    ConnectionClosed,
    ResumptionToken,
    NoEvent,
  };

  struct DataWritable_Body {
    uint64_t stream_id;
  };

  struct StopSending_Body {
    uint64_t stream_id;
    uint64_t error;
  };

  struct HeaderReady_Body {
    uint64_t stream_id;
    bool fin;
  };

  struct DataReadable_Body {
    uint64_t stream_id;
  };

  struct Reset_Body {
    uint64_t stream_id;
    uint64_t error;
    bool local;
  };

  struct PushPromise_Body {
    uint64_t push_id;
    uint64_t request_stream_id;
  };

  struct PushHeaderReady_Body {
    uint64_t push_id;
    bool fin;
  };

  struct PushDataReadable_Body {
    uint64_t push_id;
  };

  struct PushCanceled_Body {
    uint64_t push_id;
  };

  struct PushReset_Body {
    uint64_t push_id;
    uint64_t error;
  };

  struct ConnectionClosing_Body {
    CloseError error;
  };

  struct ConnectionClosed_Body {
    CloseError error;
  };

  struct ResumptionToken_Body {
    uint64_t expire_in;
  };

  Tag tag;
  union {
    DataWritable_Body data_writable;
    StopSending_Body stop_sending;
    HeaderReady_Body header_ready;
    DataReadable_Body data_readable;
    Reset_Body reset;
    PushPromise_Body push_promise;
    PushHeaderReady_Body push_header_ready;
    PushDataReadable_Body push_data_readable;
    PushCanceled_Body push_canceled;
    PushReset_Body push_reset;
    ConnectionClosing_Body connection_closing;
    ConnectionClosed_Body connection_closed;
    ResumptionToken_Body resumption_token;
  };
};

struct NeqoSecretInfo {
  bool set;
  uint16_t version;
  uint16_t cipher;
  uint16_t group;
  bool resumed;
  bool early_data;
  nsCString alpn;
  uint16_t signature_scheme;
};

struct NeqoCertificateInfo {
  nsTArray<nsTArray<uint8_t>> certs;
  bool stapled_ocsp_responses_present;
  nsTArray<nsTArray<uint8_t>> stapled_ocsp_responses;
  bool signed_cert_timestamp_present;
  nsTArray<uint8_t> signed_cert_timestamp;
};

struct Http3Stats {
  /// Total packets received, including all the bad ones.
  uintptr_t packets_rx;
  /// Duplicate packets received.
  uintptr_t dups_rx;
  /// Dropped packets or dropped garbage.
  uintptr_t dropped_rx;
  /// The number of packet that were saved for later processing.
  uintptr_t saved_datagrams;
  /// Total packets sent.
  uintptr_t packets_tx;
  /// Total number of packets that are declared lost.
  uintptr_t lost;
  /// Late acknowledgments, for packets that were declared lost already.
  uintptr_t late_ack;
  /// Acknowledgments for packets that contained data that was marked
  /// for retransmission when the PTO timer popped.
  uintptr_t pto_ack;
  /// Count PTOs. Single PTOs, 2 PTOs in a row, 3 PTOs in row, etc. are counted
  /// separately.
  uintptr_t pto_counts[16];
};

extern "C" {

nsrefcnt neqo_http3conn_addref(const NeqoHttp3Conn *conn);

nsrefcnt neqo_http3conn_release(const NeqoHttp3Conn *conn);

nsresult neqo_http3conn_new(const nsACString *origin,
                            const nsACString *alpn,
                            const nsACString *local_addr,
                            const nsACString *remote_addr,
                            uint64_t max_table_size,
                            uint16_t max_blocked_streams,
                            const nsACString *qlog_dir,
                            const NeqoHttp3Conn **result);

void neqo_http3conn_process_input(NeqoHttp3Conn *conn, const uint8_t *packet, uint32_t len);

uint64_t neqo_http3conn_process_output(NeqoHttp3Conn *conn);

bool neqo_http3conn_has_data_to_send(NeqoHttp3Conn *conn);

nsresult neqo_http3conn_get_data_to_send(NeqoHttp3Conn *conn, nsTArray<uint8_t> *packet);

void neqo_http3conn_close(NeqoHttp3Conn *conn, uint64_t error);

nsresult neqo_http3conn_fetch(NeqoHttp3Conn *conn,
                              const nsACString *method,
                              const nsACString *scheme,
                              const nsACString *host,
                              const nsACString *path,
                              const nsACString *headers,
                              uint64_t *stream_id);

nsresult neqo_htttp3conn_send_request_body(NeqoHttp3Conn *conn,
                                           uint64_t stream_id,
                                           const uint8_t *buf,
                                           uint32_t len,
                                           uint32_t *read);

nsresult neqo_http3conn_reset_stream(NeqoHttp3Conn *conn, uint64_t stream_id, uint64_t error);

nsresult neqo_http3conn_close_stream(NeqoHttp3Conn *conn, uint64_t stream_id);

nsresult neqo_http3conn_event(NeqoHttp3Conn *conn, Http3Event *ret_event, nsTArray<uint8_t> *data);

nsresult neqo_http3conn_read_response_data(NeqoHttp3Conn *conn,
                                           uint64_t stream_id,
                                           uint8_t *buf,
                                           uint32_t len,
                                           uint32_t *read,
                                           bool *fin);

nsresult neqo_http3conn_tls_info(NeqoHttp3Conn *conn, NeqoSecretInfo *sec_info);

nsresult neqo_http3conn_peer_certificate_info(NeqoHttp3Conn *conn,
                                              NeqoCertificateInfo *neqo_certs_info);

void neqo_http3conn_authenticated(NeqoHttp3Conn *conn, PRErrorCode error);

void neqo_http3conn_set_resumption_token(NeqoHttp3Conn *conn, nsTArray<uint8_t> *token);

bool neqo_http3conn_is_zero_rtt(NeqoHttp3Conn *conn);

void neqo_http3conn_get_stats(NeqoHttp3Conn *conn, Http3Stats *stats);

} // extern "C"

} // namespace net
} // namespace mozilla
