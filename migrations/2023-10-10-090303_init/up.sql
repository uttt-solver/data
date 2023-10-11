CREATE TABLE branches (
    state VARCHAR PRIMARY KEY,
    distance INT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE INDEX branches_done_distance_state ON branches (done, distance, state);

CREATE TABLE branches_next (
    current VARCHAR,
    next VARCHAR,
    CONSTRAINT branches_next_pkey PRIMARY KEY (current, next)
);

CREATE TABLE branches_prev (
    current VARCHAR,
    prev VARCHAR,
    CONSTRAINT branches_prev_pkey PRIMARY KEY (current, prev)
);

INSERT INTO branches (state, distance) VALUES ('SAAAAAAAAAAAAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('gIAAAAAAAAAAAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('iCAAAAAAAAAAAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('kAgAAAAAAAAAAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('mAIAAAAAAAAAAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('oACAAAAAAAAAAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('qAAgAAAAAAAAAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('sAAIAAAAAAAAAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('uAACAAAAAAAAAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('wAAAgAAAAAAAAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('gAAAEAAAAAAAAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('iAAABAAAAAAAAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('kAAAAQAAAAAAAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('mAAAAEAAAAAAAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('oAAAABAAAAAAAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('qAAAAAQAAAAAAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('sAAAAAEAAAAAAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('uAAAAABAAAAAAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('wAAAAAAQAAAAAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('gAAAAAACAAAAAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('iAAAAAAAgAAAAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('kAAAAAAAIAAAAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('mAAAAAAACAAAAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('oAAAAAAAAgAAAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('qAAAAAAAAIAAAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('sAAAAAAAACAAAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('uAAAAAAAAAgAAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('wAAAAAAAAAIAAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('gAAAAAAAAABAAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('iAAAAAAAAAAQAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('kAAAAAAAAAAEAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('mAAAAAAAAAABAAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('oAAAAAAAAAAAQAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('qAAAAAAAAAAAEAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('sAAAAAAAAAAABAAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('uAAAAAAAAAAAAQAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('wAAAAAAAAAAAAEAAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('gAAAAAAAAAAAAAgAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('iAAAAAAAAAAAAAIAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('kAAAAAAAAAAAAACAAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('mAAAAAAAAAAAAAAgAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('oAAAAAAAAAAAAAAIAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('qAAAAAAAAAAAAAACAAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('sAAAAAAAAAAAAAAAgAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('uAAAAAAAAAAAAAAAIAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('wAAAAAAAAAAAAAAACAAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('gAAAAAAAAAAAAAAAAQAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('iAAAAAAAAAAAAAAAAEAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('kAAAAAAAAAAAAAAAABAAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('mAAAAAAAAAAAAAAAAAQAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('oAAAAAAAAAAAAAAAAAEAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('qAAAAAAAAAAAAAAAAABAAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('sAAAAAAAAAAAAAAAAAAQAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('uAAAAAAAAAAAAAAAAAAEAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('wAAAAAAAAAAAAAAAAAABAAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('gAAAAAAAAAAAAAAAAAAAIAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('iAAAAAAAAAAAAAAAAAAACAAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('kAAAAAAAAAAAAAAAAAAAAgAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('mAAAAAAAAAAAAAAAAAAAAIAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('oAAAAAAAAAAAAAAAAAAAACAAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('qAAAAAAAAAAAAAAAAAAAAAgAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('sAAAAAAAAAAAAAAAAAAAAAIAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('uAAAAAAAAAAAAAAAAAAAAACAAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('wAAAAAAAAAAAAAAAAAAAAAAgAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('gAAAAAAAAAAAAAAAAAAAAAAEAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('iAAAAAAAAAAAAAAAAAAAAAABAAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('kAAAAAAAAAAAAAAAAAAAAAAAQAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('mAAAAAAAAAAAAAAAAAAAAAAAEAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('oAAAAAAAAAAAAAAAAAAAAAAABAAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('qAAAAAAAAAAAAAAAAAAAAAAAAQAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('sAAAAAAAAAAAAAAAAAAAAAAAAEAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('uAAAAAAAAAAAAAAAAAAAAAAAABAAAA', 0);
INSERT INTO branches (state, distance) VALUES ('wAAAAAAAAAAAAAAAAAAAAAAAAAQAAA', 0);
INSERT INTO branches (state, distance) VALUES ('gAAAAAAAAAAAAAAAAAAAAAAAAACAAA', 0);
INSERT INTO branches (state, distance) VALUES ('iAAAAAAAAAAAAAAAAAAAAAAAAAAgAA', 0);
INSERT INTO branches (state, distance) VALUES ('kAAAAAAAAAAAAAAAAAAAAAAAAAAIAA', 0);
INSERT INTO branches (state, distance) VALUES ('mAAAAAAAAAAAAAAAAAAAAAAAAAACAA', 0);
INSERT INTO branches (state, distance) VALUES ('oAAAAAAAAAAAAAAAAAAAAAAAAAAAgA', 0);
INSERT INTO branches (state, distance) VALUES ('qAAAAAAAAAAAAAAAAAAAAAAAAAAAIA', 0);
INSERT INTO branches (state, distance) VALUES ('sAAAAAAAAAAAAAAAAAAAAAAAAAAACA', 0);
INSERT INTO branches (state, distance) VALUES ('uAAAAAAAAAAAAAAAAAAAAAAAAAAAAg', 0);
INSERT INTO branches (state, distance) VALUES ('wAAAAAAAAAAAAAAAAAAAAAAAAAAAAI', 0);
