INSERT INTO users (id, first_name, last_name, headline, role) VALUES
('00000000-0000-0000-0000-000000000001', 'Danil',  'Suiagin', 'Fullstack Ninja',   'ADMIN'),
('00000000-0000-0000-0000-000000000002', 'Alice',  'Doe',     'QA Lead',        'SUPPORT'),
('00000000-0000-0000-0000-000000000003', 'Bob',    'Smith',   'Frontend Dev',   'DEVELOPER'),
('00000000-0000-0000-0000-000000000004', 'Carol',  'Jones',   NULL,             'SUPPORT'),
('00000000-0000-0000-0000-000000000005', 'Eve',    'Black',   'UX Designer',    'ADMIN');

INSERT INTO tasks (title, category, description, creator_id, assigned_to) VALUES
('Login flow fails',            'BUG',      'OAuth callback returns 500','00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000003'),
('Broken hero image',           'BUG',      'Homepage banner 404s','00000000-0000-0000-0000-000000000003', '00000000-0000-0000-0000-000000000005'),
('Refactor payment module',     'TASK',     'Split into micro-services','00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000004'),
('Add dark-mode toggle',        'TASK',     'Improve accessibility','00000000-0000-0000-0000-000000000005', '00000000-0000-0000-0000-000000000003'),
('Fix typo in FAQ',             'BUG',      'Wrong pricing info','00000000-0000-0000-0000-000000000004', '00000000-0000-0000-0000-000000000002'),
('Optimize DB indices',         'RESEARCH', 'Investigate slow query on tasks','00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000002'),
('Mobile menu CSS glitch',      'BUG',      'Off-canvas not hidden','00000000-0000-0000-0000-000000000003', '00000000-0000-0000-0000-000000000004'),
('Email notifications MVP',     'TASK',     'Notify assignee on update','00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000005'),
('Investigate memory leak',     'RESEARCH', 'High RSS in production pod','00000000-0000-0000-0000-000000000005', '00000000-0000-0000-0000-000000000001'),
('500 on /profile',             'BUG',      'Null headline crash','00000000-0000-0000-0000-000000000004', '00000000-0000-0000-0000-000000000001');
