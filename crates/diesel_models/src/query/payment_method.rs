use async_bb8_diesel::AsyncRunQueryDsl;
#[cfg(feature = "v1")]
use diesel::Table;
use diesel::{
    associations::HasTable, debug_query, pg::Pg, BoolExpressionMethods, ExpressionMethods, QueryDsl,
};
use error_stack::ResultExt;

use super::generics;
#[cfg(feature = "v1")]
use crate::schema::payment_methods::dsl;
#[cfg(feature = "v2")]
use crate::schema_v2::payment_methods::dsl::{self, id as pm_id};
use crate::{
    enums as storage_enums, errors,
    payment_method::{self, PaymentMethod, PaymentMethodNew},
    PgPooledConn, StorageResult,
};

impl PaymentMethodNew {
    pub async fn insert(self, conn: &PgPooledConn) -> StorageResult<PaymentMethod> {
        generics::generic_insert(conn, self).await
    }
}

#[cfg(feature = "v1")]
impl PaymentMethod {
    pub async fn delete_by_payment_method_id(
        conn: &PgPooledConn,
        payment_method_id: String,
    ) -> StorageResult<Self> {
        generics::generic_delete_one_with_result::<<Self as HasTable>::Table, _, Self>(
            conn,
            dsl::payment_method_id.eq(payment_method_id),
        )
        .await
    }

    pub async fn delete_by_merchant_id_payment_method_id(
        conn: &PgPooledConn,
        merchant_id: &common_utils::id_type::MerchantId,
        payment_method_id: &str,
    ) -> StorageResult<Self> {
        generics::generic_delete_one_with_result::<<Self as HasTable>::Table, _, Self>(
            conn,
            dsl::merchant_id
                .eq(merchant_id.to_owned())
                .and(dsl::payment_method_id.eq(payment_method_id.to_owned())),
        )
        .await
    }

    pub async fn find_by_locker_id(conn: &PgPooledConn, locker_id: &str) -> StorageResult<Self> {
        generics::generic_find_one::<<Self as HasTable>::Table, _, _>(
            conn,
            dsl::locker_id.eq(locker_id.to_owned()),
        )
        .await
    }

    pub async fn find_by_payment_method_id(
        conn: &PgPooledConn,
        payment_method_id: &str,
    ) -> StorageResult<Self> {
        generics::generic_find_one::<<Self as HasTable>::Table, _, _>(
            conn,
            dsl::payment_method_id.eq(payment_method_id.to_owned()),
        )
        .await
    }

    pub async fn find_by_merchant_id(
        conn: &PgPooledConn,
        merchant_id: &common_utils::id_type::MerchantId,
    ) -> StorageResult<Vec<Self>> {
        generics::generic_filter::<
            <Self as HasTable>::Table,
            _,
            <<Self as HasTable>::Table as Table>::PrimaryKey,
            _,
        >(
            conn,
            dsl::merchant_id.eq(merchant_id.to_owned()),
            None,
            None,
            None,
        )
        .await
    }

    pub async fn find_by_customer_id_merchant_id(
        conn: &PgPooledConn,
        customer_id: &common_utils::id_type::CustomerId,
        merchant_id: &common_utils::id_type::MerchantId,
        limit: Option<i64>,
    ) -> StorageResult<Vec<Self>> {
        generics::generic_filter::<<Self as HasTable>::Table, _, _, _>(
            conn,
            dsl::customer_id
                .eq(customer_id.to_owned())
                .and(dsl::merchant_id.eq(merchant_id.to_owned())),
            limit,
            None,
            Some(dsl::last_used_at.desc()),
        )
        .await
    }

    pub async fn get_count_by_customer_id_merchant_id_status(
        conn: &PgPooledConn,
        customer_id: &common_utils::id_type::CustomerId,
        merchant_id: &common_utils::id_type::MerchantId,
        status: common_enums::PaymentMethodStatus,
    ) -> StorageResult<i64> {
        let filter = <Self as HasTable>::table()
            .count()
            .filter(
                dsl::customer_id
                    .eq(customer_id.to_owned())
                    .and(dsl::merchant_id.eq(merchant_id.to_owned()))
                    .and(dsl::status.eq(status.to_owned())),
            )
            .into_boxed();

        router_env::logger::debug!(query = %debug_query::<Pg, _>(&filter).to_string());

        generics::db_metrics::track_database_call::<<Self as HasTable>::Table, _, _>(
            filter.get_result_async::<i64>(conn),
            generics::db_metrics::DatabaseOperation::Count,
        )
        .await
        .change_context(errors::DatabaseError::Others)
        .attach_printable("Failed to get a count of payment methods")
    }

    pub async fn get_count_by_merchant_id_status(
        conn: &PgPooledConn,
        merchant_id: &common_utils::id_type::MerchantId,
        status: common_enums::PaymentMethodStatus,
    ) -> StorageResult<i64> {
        let query = <Self as HasTable>::table().count().filter(
            dsl::merchant_id
                .eq(merchant_id.to_owned())
                .and(dsl::status.eq(status.to_owned())),
        );

        router_env::logger::debug!(query = %debug_query::<Pg, _>(&query).to_string());

        generics::db_metrics::track_database_call::<<Self as HasTable>::Table, _, _>(
            query.get_result_async::<i64>(conn),
            generics::db_metrics::DatabaseOperation::Count,
        )
        .await
        .change_context(errors::DatabaseError::Others)
        .attach_printable("Failed to get a count of payment methods")
    }

    pub async fn find_by_customer_id_merchant_id_status(
        conn: &PgPooledConn,
        customer_id: &common_utils::id_type::CustomerId,
        merchant_id: &common_utils::id_type::MerchantId,
        status: storage_enums::PaymentMethodStatus,
        limit: Option<i64>,
    ) -> StorageResult<Vec<Self>> {
        generics::generic_filter::<<Self as HasTable>::Table, _, _, _>(
            conn,
            dsl::customer_id
                .eq(customer_id.to_owned())
                .and(dsl::merchant_id.eq(merchant_id.to_owned()))
                .and(dsl::status.eq(status)),
            limit,
            None,
            Some(dsl::last_used_at.desc()),
        )
        .await
    }

    pub async fn update_with_payment_method_id(
        self,
        conn: &PgPooledConn,
        payment_method: payment_method::PaymentMethodUpdateInternal,
    ) -> StorageResult<Self> {
        match generics::generic_update_with_unique_predicate_get_result::<
            <Self as HasTable>::Table,
            _,
            _,
            _,
        >(
            conn,
            dsl::payment_method_id.eq(self.payment_method_id.to_owned()),
            payment_method,
        )
        .await
        {
            Err(error) => match error.current_context() {
                errors::DatabaseError::NoFieldsToUpdate => Ok(self),
                _ => Err(error),
            },
            result => result,
        }
    }
}

#[cfg(feature = "v2")]
impl PaymentMethod {
    pub async fn find_by_id(
        conn: &PgPooledConn,
        id: &common_utils::id_type::GlobalPaymentMethodId,
    ) -> StorageResult<Self> {
        generics::generic_find_one::<<Self as HasTable>::Table, _, _>(conn, pm_id.eq(id.to_owned()))
            .await
    }

    pub async fn find_by_global_customer_id_merchant_id_status(
        conn: &PgPooledConn,
        customer_id: &common_utils::id_type::GlobalCustomerId,
        merchant_id: &common_utils::id_type::MerchantId,
        status: storage_enums::PaymentMethodStatus,
        limit: Option<i64>,
    ) -> StorageResult<Vec<Self>> {
        generics::generic_filter::<<Self as HasTable>::Table, _, _, _>(
            conn,
            dsl::customer_id
                .eq(customer_id.to_owned())
                .and(dsl::merchant_id.eq(merchant_id.to_owned()))
                .and(dsl::status.eq(status)),
            limit,
            None,
            Some(dsl::last_used_at.desc()),
        )
        .await
    }

    pub async fn find_by_global_customer_id(
        conn: &PgPooledConn,
        customer_id: &common_utils::id_type::GlobalCustomerId,
        limit: Option<i64>,
    ) -> StorageResult<Vec<Self>> {
        generics::generic_filter::<<Self as HasTable>::Table, _, _, _>(
            conn,
            dsl::customer_id.eq(customer_id.to_owned()),
            limit,
            None,
            Some(dsl::last_used_at.desc()),
        )
        .await
    }

    pub async fn update_with_id(
        self,
        conn: &PgPooledConn,
        payment_method: payment_method::PaymentMethodUpdateInternal,
    ) -> StorageResult<Self> {
        match generics::generic_update_with_unique_predicate_get_result::<
            <Self as HasTable>::Table,
            _,
            _,
            _,
        >(conn, pm_id.eq(self.id.to_owned()), payment_method)
        .await
        {
            Err(error) => match error.current_context() {
                errors::DatabaseError::NoFieldsToUpdate => Ok(self),
                _ => Err(error),
            },
            result => result,
        }
    }

    pub async fn find_by_fingerprint_id(
        conn: &PgPooledConn,
        fingerprint_id: &str,
    ) -> StorageResult<Self> {
        generics::generic_find_one::<<Self as HasTable>::Table, _, _>(
            conn,
            dsl::locker_fingerprint_id.eq(fingerprint_id.to_owned()),
        )
        .await
    }

    pub async fn get_count_by_merchant_id_status(
        conn: &PgPooledConn,
        merchant_id: &common_utils::id_type::MerchantId,
        status: common_enums::PaymentMethodStatus,
    ) -> StorageResult<i64> {
        let query = <Self as HasTable>::table().count().filter(
            dsl::merchant_id
                .eq(merchant_id.to_owned())
                .and(dsl::status.eq(status.to_owned())),
        );

        router_env::logger::debug!(query = %debug_query::<Pg, _>(&query).to_string());

        generics::db_metrics::track_database_call::<<Self as HasTable>::Table, _, _>(
            query.get_result_async::<i64>(conn),
            generics::db_metrics::DatabaseOperation::Count,
        )
        .await
        .change_context(errors::DatabaseError::Others)
        .attach_printable("Failed to get a count of payment methods")
    }
}
