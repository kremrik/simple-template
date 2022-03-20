from tool import template

import unittest


class test_line_is_var(unittest.TestCase):
    def test_not_var(self):
        line = "foo"
        expect = False
        actual = template.line_is_var(line)
        self.assertEqual(expect, actual)

    def test_is_var(self):
        line = "{{variable}}"
        expect = True
        actual = template.line_is_var(line)
        self.assertEqual(expect, actual)

    def test_is_var_with_indent(self):
        line = "    {{variable}}"
        expect = True
        actual = template.line_is_var(line)
        self.assertEqual(expect, actual)

    def test_is_var_with_comment(self):
        line = "{{ variable /* COMMENT */ }}"
        expect = True
        actual = template.line_is_var(line)
        self.assertEqual(expect, actual)


class test_get_indent(unittest.TestCase):
    def test_none(self):
        line = "{{variable}}"
        expect = 0
        actual = template.get_indent(line)
        self.assertEqual(expect, actual)

    def test_newline(self):
        line = "  {{variable}}"
        expect = 2
        actual = template.get_indent(line)
        self.assertEqual(expect, actual)

    def test_inline(self):
        line = '<a href="{{variable}}">...</a>'
        expect = 0
        actual = template.get_indent(line)
        self.assertEqual(expect, actual)

    def test_indented_inline(self):
        line = '    <a href="{{variable}}">...</a>'
        expect = 0
        actual = template.get_indent(line)
        self.assertEqual(expect, actual)


class test_get_var_name(unittest.TestCase):
    def test_no_comment(self):
        line = "{{ variable }}"
        expect = "variable"
        actual = template.get_var_name(line)
        self.assertEqual(expect, actual)

    def test_with_comment(self):
        line = "{{ variable /* COMMENT */ }}"
        expect = "variable"
        actual = template.get_var_name(line)
        self.assertEqual(expect, actual)


class test_get_var_comment(unittest.TestCase):
    def test_no_comment(self):
        line = "{{ variable }}"
        expect = None
        actual = template.get_var_comment(line)
        self.assertEqual(expect, actual)

    def test_with_comment(self):
        line = "{{ variable /* COMMENT */ }}"
        expect = "COMMENT"
        actual = template.get_var_comment(line)
        self.assertEqual(expect, actual)

    def test_with_complex_comment(self):
        line = "{{ variable /* /path/to/head.html template */ }}"
        expect = "/path/to/head.html template"
        actual = template.get_var_comment(line)
        self.assertEqual(expect, actual)
