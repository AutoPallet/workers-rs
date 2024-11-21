use wasm_bindgen::prelude::*;

use crate::types::DurableObjectTransaction;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends=js_sys::Object)]
    pub type DurableObjectStorage;

    #[wasm_bindgen(method, catch)]
    pub fn get(this: &DurableObjectStorage, key: &str) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(method, catch, js_name=get)]
    pub fn get_multiple(
        this: &DurableObjectStorage,
        keys: Vec<JsValue>,
    ) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub fn put(
        this: &DurableObjectStorage,
        key: &str,
        value: JsValue,
    ) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(method, catch, js_name=put)]
    pub fn put_multiple(
        this: &DurableObjectStorage,
        value: JsValue,
    ) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub fn delete(this: &DurableObjectStorage, key: &str) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(method, catch, js_name=delete)]
    pub fn delete_multiple(
        this: &DurableObjectStorage,
        keys: Vec<JsValue>,
    ) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(method, catch, js_name=deleteAll)]
    pub fn delete_all(this: &DurableObjectStorage) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub fn list(this: &DurableObjectStorage) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(method, catch, js_name=list)]
    pub fn list_with_options(
        this: &DurableObjectStorage,
        options: js_sys::Object,
    ) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub fn transaction(
        this: &DurableObjectStorage,
        closure: &Closure<dyn FnMut(DurableObjectTransaction) -> js_sys::Promise>,
    ) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(method, catch, js_name=getAlarm)]
    pub fn get_alarm(
        this: &DurableObjectStorage,
        options: js_sys::Object,
    ) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(method, catch, js_name=setAlarm)]
    pub fn set_alarm(
        this: &DurableObjectStorage,
        scheduled_time: js_sys::Date,
        options: js_sys::Object,
    ) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(method, catch, js_name=deleteAlarm)]
    pub fn delete_alarm(
        this: &DurableObjectStorage,
        options: js_sys::Object,
    ) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(method, getter)]
    pub fn sql(this: &DurableObjectStorage) -> DurableObjectStorageSql;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type DurableObjectStorageSql;

    #[wasm_bindgen(method, catch, variadic)]
    pub fn exec(
        this: &DurableObjectStorageSql,
        query: &str,
        bindings: js_sys::Array,
    ) -> Result<SqlStorageCursor, JsValue>;

    #[wasm_bindgen(method, getter, catch, js_name = databaseSize)]
    pub fn database_size(this: &DurableObjectStorageSql) -> Result<u32, JsValue>;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type SqlStorageCursor;

    /// Retrieves the next row in the cursor.
    ///
    /// # Returns
    /// An object with `done` and `value` properties. If `done` is `true`, the cursor is exhausted.
    #[wasm_bindgen(method, catch)]
    pub fn next(this: &SqlStorageCursor) -> Result<JsValue, JsValue>;

    /// Converts the remaining rows in the cursor to an array of row objects.
    ///
    /// # Returns
    /// An array of row objects.
    #[wasm_bindgen(method,  js_name = toArray)]
    pub fn to_array(this: &SqlStorageCursor) -> js_sys::Array;

    /// Retrieves exactly one row from the cursor.
    ///
    /// # Returns
    /// A single row object if exactly one row exists; otherwise, throws an error.
    #[wasm_bindgen(method, catch)]
    pub fn one(this: &SqlStorageCursor) -> Result<JsValue, JsValue>;

    /// Retrieves a raw iterator that returns rows as arrays of column values.
    ///
    /// # Returns
    /// A `SqlStorageCursorRawIterator`.
    #[wasm_bindgen(method)]
    pub fn raw(this: &SqlStorageCursor) -> SqlStorageCursorRawIterator;

    /// Gets the column names of the query result.
    ///
    /// # Returns
    /// An array of column name strings.
    #[wasm_bindgen(method, getter, js_name = columnNames)]
    pub fn column_names(this: &SqlStorageCursor) -> Vec<String>;

    /// Gets the number of rows read so far.
    ///
    /// # Returns
    /// The number of rows read.
    #[wasm_bindgen(method, getter, js_name = rowsRead)]
    pub fn rows_read(this: &SqlStorageCursor) -> u32;

    /// Gets the number of rows written so far.
    ///
    /// # Returns
    /// The number of rows written.
    #[wasm_bindgen(method, getter, js_name = rowsWritten)]
    pub fn rows_written(this: &SqlStorageCursor) -> u32;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type SqlStorageCursorRawIterator;

    /// Retrieves the next raw row in the iterator.
    ///
    /// # Returns
    /// An object with `done` and `value` properties. If `done` is `true`, the iterator is exhausted.
    #[wasm_bindgen(method, catch)]
    pub fn next(this: &SqlStorageCursorRawIterator) -> Result<JsValue, JsValue>;

    /// Converts the remaining raw rows in the iterator to an array.
    ///
    /// # Returns
    /// An array of raw row arrays.
    #[wasm_bindgen(method, js_name = toArray)]
    pub fn to_array(this: &SqlStorageCursorRawIterator) -> js_sys::Array;

    /// Retrieves exactly one raw row from the iterator.
    ///
    /// # Returns
    /// A single raw row array if exactly one row exists; otherwise, throws an error.
    #[wasm_bindgen(method, catch)]
    pub fn one(this: &SqlStorageCursorRawIterator) -> Result<JsValue, JsValue>;
}
