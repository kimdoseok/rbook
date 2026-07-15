-- Table: yug_usergroups

CREATE TABLE IF NOT EXISTS yug_usergroups (
    ID bigserial primary key,
	yug_code character varying(32) unique NOT NULL,
    yug_name text default '',
    yug_active boolean DEFAULT true,
    yug_memo text default '',
    yug_system boolean DEFAULT false,
	CreatedAt timestamp without time zone DEFAULT now(),
	UpdatedAt timestamp without time zone DEFAULT now(),
	DeletedAt timestamp without time zone
);


CREATE OR REPLACE FUNCTION prevent_delete_if_system()
RETURNS trigger AS $$
BEGIN
    IF OLD.yug_system = true THEN
        RAISE EXCEPTION 'Delete blocked: this row cannot be deleted.';
    END IF;

    RETURN OLD; -- required for BEFORE DELETE
END;
$$ LANGUAGE plpgsql;


DROP TRIGGER IF EXISTS yug_prevent_delete ON yug_usergroups;
CREATE TRIGGER yug_prevent_delete
BEFORE DELETE ON yug_usergroups
FOR EACH ROW
EXECUTE FUNCTION prevent_delete_if_system();


INSERT INTO yug_usergroups (yug_code, yug_name, yug_memo, yug_active, yug_system) VALUES 
('ADMINS','Administrators','',true,true) ON CONFLICT (yug_code) DO NOTHING;
INSERT INTO yug_usergroups (yug_code, yug_name, yug_memo, yug_active, yug_system) VALUES 
('USERS','Users','',true,false) ON CONFLICT (yug_code) DO NOTHING;
