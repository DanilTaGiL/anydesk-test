INSERT INTO users (id, first_name, last_name, headline, role) VALUES
('00000000-0000-0000-0000-000000000001', 'Danil',  'Suiagin', 'Backend Ninja', 'ADMIN'),
('00000000-0000-0000-0000-000000000002', 'Alice',  'Doe',     'QA Lead',      'SUPPORT'),
('00000000-0000-0000-0000-000000000003', 'Bob',    'Smith',   'Frontend Dev', 'SUPPORT'),
('00000000-0000-0000-0000-000000000004', 'Carol',  'Jones',   NULL,           'SUPPORT'),
('00000000-0000-0000-0000-000000000005', 'Eve',    'Black',   'UX Designer',  'ADMIN');

-- ===== Tasks (10) =====
INSERT INTO tasks (title, category, description, creator_id, assigned_to) VALUES
('Bug in login flow',            'BUG',  'OAuth callback fails', '00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000003'),
('Broken image on homepage',     'BUG',  'Hero banner 404s',      '00000000-0000-0000-0000-000000000003', '00000000-0000-0000-0000-000000000005'),
('Refactor payment module',      'TASK', 'Split into microservices','00000000-0000-0000-0000-000000000001','00000000-0000-0000-0000-000000000004'),
('Add dark-mode toggle',         'TASK', 'Improve accessibility','00000000-0000-0000-0000-000000000005','00000000-0000-0000-0000-000000000003'),
('Fix typo in FAQ',              'BUG',  'Wrong pricing info',   '00000000-0000-0000-0000-000000000004','00000000-0000-0000-0000-000000000002'),
('Optimize DB indices',          'TASK', 'Slow query on tasks',  '00000000-0000-0000-0000-000000000001','00000000-0000-0000-0000-000000000002'),
('Broken CSS in mobile menu',    'BUG',  'Off-canvas not hidden','00000000-0000-0000-0000-000000000003','00000000-0000-0000-0000-000000000004'),
('Implement email notifications','TASK', 'Notify on assignment', '00000000-0000-0000-0000-000000000002','00000000-0000-0000-0000-000000000005'),
('Investigate memory leak',      'TASK', 'High RSS in prod',     '00000000-0000-0000-0000-000000000005','00000000-0000-0000-0000-000000000001'),
('Fix 500 on /profile',          'BUG',  'Null headline crash',  '00000000-0000-0000-0000-000000000004','00000000-0000-0000-0000-000000000001');
