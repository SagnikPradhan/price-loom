CREATE TYPE source AS ENUM ('nse');

CREATE TYPE instrument_type AS ENUM ('CUR', 'CDF', 'CDO', 'IRF', 'IRT', 'IRO', 'STK', 'COM', 'COF', 'COO', 'FUO', 'STF', 'STO', 'IDF', 'IDO');

CREATE TYPE instrument_segment AS ENUM ('EQ', 'FO', 'CD', 'IR', 'COM');

CREATE TABLE source_file (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
	source source NOT NULL,
	date DATE NOT NULL,
	key TEXT NOT NULL,
	checksum TEXT NOT NULL
);

CREATE TABLE instrument (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
	segment instrument_segment NOT NULL,
	source source NOT NULL,
	instrument_type instrument_type NOT NULL,
	instrument_id TEXT,
	isin TEXT NOT NULL,
	ticker_symbol TEXT NOT NULL,
	security_series TEXT NOT NULL,
	instrument_name TEXT
);

CREATE TABLE price (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
	file_id UUID NOT NULL REFERENCES source_file (id) ON DELETE CASCADE,
	instrument_id UUID REFERENCES instrument (id) ON DELETE CASCADE,
	trade_date DATE NOT NULL,
	business_date DATE,
	open_price DOUBLE PRECISION NOT NULL,
	high_price DOUBLE PRECISION NOT NULL,
	low_price DOUBLE PRECISION NOT NULL,
	close_price DOUBLE PRECISION NOT NULL,
	last_price DOUBLE PRECISION NOT NULL,
	previous_close_price DOUBLE PRECISION NOT NULL,
	total_traded_volume BIGINT NOT NULL,
	total_traded_value DOUBLE PRECISION NOT NULL,
	total_number_of_trades BIGINT NOT NULL,
	session_id TEXT,
	market_lot_size BIGINT,
	remarks TEXT
);

CREATE UNIQUE INDEX "source_file_unique_idx" ON source_file USING btree (source, date);

CREATE UNIQUE INDEX "instrument_unique_idx" ON instrument USING btree (source, instrument_type, isin);

CREATE UNIQUE INDEX "price_unique_idx" ON price USING btree (instrument_id, trade_date);