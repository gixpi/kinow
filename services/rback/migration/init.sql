INSERT INTO permissions VALUES 
('user_account.list_sessions', 'List User Sessions'),
('user_user.list_users', 'List Users'),
('user_user.change_user_status', 'Change User Status'),
('user_user.delete_user', 'Delete User');

INSERT INTO roles VALUES 
('owner','Owner Role');

INSERT INTO role_permissions VALUES
('owner', 'user_account.list_sessions'),
('owner', 'user_user.list_users'),
('owner', 'user_user.change_user_status'),
('owner', 'user_user.delete_user');
