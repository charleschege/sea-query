use crate::{backend::SchemaBuilder, prepare::*, types::*, ColumnDef, SchemaStatementBuilder};

/// Alter a table
///
/// # Examples
///
/// ```
/// use sea_query::{tests_cfg::*, *};
///
/// let table = Table::alter()
///     .table(Font::Table)
///     .add_column(
///         ColumnDef::new(Alias::new("new_col"))
///             .integer()
///             .not_null()
///             .default(100),
///     )
///     .to_owned();
///
/// assert_eq!(
///     table.to_string(MysqlQueryBuilder),
///     r#"ALTER TABLE `font` ADD COLUMN `new_col` int NOT NULL DEFAULT 100"#
/// );
/// assert_eq!(
///     table.to_string(PostgresQueryBuilder),
///     r#"ALTER TABLE "font" ADD COLUMN "new_col" integer NOT NULL DEFAULT 100"#
/// );
/// assert_eq!(
///     table.to_string(SqliteQueryBuilder),
///     r#"ALTER TABLE "font" ADD COLUMN "new_col" integer NOT NULL DEFAULT 100"#,
/// );
/// ```
#[derive(Debug, Clone)]
pub struct TableAlterStatement {
    pub(crate) table: Option<DynIden>,
    pub(crate) alter_option: Option<TableAlterOption>,
}

/// All available table alter options
#[derive(Debug, Clone)]
pub enum TableAlterOption {
    AddColumn(ColumnDef),
    ModifyColumn(ColumnDef),
    RenameColumn(DynIden, DynIden),
    DropColumn(DynIden),
}

impl Default for TableAlterStatement {
    fn default() -> Self {
        Self::new()
    }
}

impl TableAlterStatement {
    /// Construct alter table statement
    pub fn new() -> Self {
        Self {
            table: None,
            alter_option: None,
        }
    }

    /// Set table name
    pub fn table<T: 'static>(&mut self, table: T) -> &mut Self
    where
        T: Iden,
    {
        self.table = Some(SeaRc::new(table));
        self
    }

    /// Add a column to an existing table
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let table = Table::alter()
    ///     .table(Font::Table)
    ///     .add_column(
    ///         ColumnDef::new(Alias::new("new_col"))
    ///             .integer()
    ///             .not_null()
    ///             .default(100),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     table.to_string(MysqlQueryBuilder),
    ///     r#"ALTER TABLE `font` ADD COLUMN `new_col` int NOT NULL DEFAULT 100"#
    /// );
    /// assert_eq!(
    ///     table.to_string(PostgresQueryBuilder),
    ///     r#"ALTER TABLE "font" ADD COLUMN "new_col" integer NOT NULL DEFAULT 100"#
    /// );
    /// assert_eq!(
    ///     table.to_string(SqliteQueryBuilder),
    ///     r#"ALTER TABLE "font" ADD COLUMN "new_col" integer NOT NULL DEFAULT 100"#,
    /// );
    /// ```
    pub fn add_column(&mut self, column_def: &mut ColumnDef) -> &mut Self {
        self.alter_option(TableAlterOption::AddColumn(column_def.take()))
    }

    /// Modify a column in an existing table
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let table = Table::alter()
    ///     .table(Font::Table)
    ///     .modify_column(
    ///         ColumnDef::new(Alias::new("new_col"))
    ///             .big_integer()
    ///             .default(999),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     table.to_string(MysqlQueryBuilder),
    ///     r#"ALTER TABLE `font` MODIFY COLUMN `new_col` bigint DEFAULT 999"#
    /// );
    /// assert_eq!(
    ///     table.to_string(PostgresQueryBuilder),
    ///     vec![
    ///         r#"ALTER TABLE "font""#,
    ///         r#"ALTER COLUMN "new_col" TYPE bigint,"#,
    ///         r#"ALTER COLUMN "new_col" SET DEFAULT 999"#,
    ///     ]
    ///     .join(" ")
    /// );
    /// // Sqlite not support modifying table column
    /// ```
    pub fn modify_column(&mut self, column_def: &mut ColumnDef) -> &mut Self {
        self.alter_option(TableAlterOption::ModifyColumn(column_def.take()))
    }

    /// Rename a column in an existing table
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let table = Table::alter()
    ///     .table(Font::Table)
    ///     .rename_column(Alias::new("new_col"), Alias::new("new_column"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     table.to_string(MysqlQueryBuilder),
    ///     r#"ALTER TABLE `font` RENAME COLUMN `new_col` TO `new_column`"#
    /// );
    /// assert_eq!(
    ///     table.to_string(PostgresQueryBuilder),
    ///     r#"ALTER TABLE "font" RENAME COLUMN "new_col" TO "new_column""#
    /// );
    /// assert_eq!(
    ///     table.to_string(SqliteQueryBuilder),
    ///     r#"ALTER TABLE "font" RENAME COLUMN "new_col" TO "new_column""#
    /// );
    /// ```
    pub fn rename_column<T: 'static, R: 'static>(&mut self, from_name: T, to_name: R) -> &mut Self
    where
        T: Iden,
        R: Iden,
    {
        self.alter_option(TableAlterOption::RenameColumn(
            SeaRc::new(from_name),
            SeaRc::new(to_name),
        ))
    }

    /// Add a column to existing table
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let table = Table::alter()
    ///     .table(Font::Table)
    ///     .drop_column(Alias::new("new_column"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     table.to_string(MysqlQueryBuilder),
    ///     r#"ALTER TABLE `font` DROP COLUMN `new_column`"#
    /// );
    /// assert_eq!(
    ///     table.to_string(PostgresQueryBuilder),
    ///     r#"ALTER TABLE "font" DROP COLUMN "new_column""#
    /// );
    /// // Sqlite not support modifying table column
    /// ```
    pub fn drop_column<T: 'static>(&mut self, col_name: T) -> &mut Self
    where
        T: Iden,
    {
        self.alter_option(TableAlterOption::DropColumn(SeaRc::new(col_name)))
    }

    fn alter_option(&mut self, alter_option: TableAlterOption) -> &mut Self {
        self.alter_option = Some(alter_option);
        self
    }

    pub fn take(&mut self) -> Self {
        Self {
            table: self.table.take(),
            alter_option: self.alter_option.take(),
        }
    }
}

impl SchemaStatementBuilder for TableAlterStatement {
    fn build<T: SchemaBuilder>(&self, schema_builder: T) -> String {
        let mut sql = SqlWriter::new();
        schema_builder.prepare_table_alter_statement(self, &mut sql);
        sql.result()
    }

    fn build_any(&self, schema_builder: &dyn SchemaBuilder) -> String {
        let mut sql = SqlWriter::new();
        schema_builder.prepare_table_alter_statement(self, &mut sql);
        sql.result()
    }
}
