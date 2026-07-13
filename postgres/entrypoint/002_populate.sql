CREATE TABLE IF NOT EXISTS yug_usergroups (
    ID bigserial primary key,
	yug_code character varying(16) unique NOT NULL,
    yug_name text default '',
    yug_active boolean DEFAULT true,
    yug_memo text default '',
    yug_system boolean DEFAULT false,
	CreatedAt timestamp without time zone DEFAULT now(),
	UpdatedAt timestamp without time zone DEFAULT now(),
	DeletedAt timestamp without time zone
);

INSERT INTO yug_usergroups (yug_code, yug_name, yug_memo, yug_active, yug_system) VALUES 
('ADMINS','Administrators','',true,true) ON CONFLICT (yug_code) DO NOTHING;
INSERT INTO yug_usergroups (yug_code, yug_name, yug_memo, yug_active, yug_system) VALUES 
('USERS','Users','',true,false) ON CONFLICT (yug_code) DO NOTHING;
