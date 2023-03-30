import sys
import os
import json
import time
import cv2 as cv


class Prog:
    def __init__(self):
        parent = os.path.dirname(os.path.realpath(__file__))
        path = os.path.join(parent, 'config.json')
        self.config = json.load(open(path))

        dir_ext = time.strftime("%d_%H:%M:%S")
        self.dir_log = os.path.join(self.config['log_dir'], dir_ext)
        self.dir_inter = self.config['inter_dir']
        self.dir_out = self.config['out_dir']
        for path in [self.dir_log, self.dir_inter, self.dir_out]:
            os.makedirs(path, exist_ok=True)

        self.timer = time.time()

    def __del__(self):
        if self.config['options']['timer']:
            self.timer_end(self.timer, 'prog ')

    def path_jpg(self, root, name):
        return os.path.join(root, self.config['name'] + '_' + name + '.jpg')

    def log_picture(self, img, message):
        if self.config['options_pre']['log_pictures']:
            cv.imwrite(self.path_jpg(self.dir_log, 'pre_' + message), img)

    def log(self, message):
        if self.config['options_pre']['log_text']:
            print('\033[1mLOG\033[0m: ' + message, file=sys.stderr)

    def timer_start(self):
        if self.config['options']['timer']:
            return time.time()
        return None

    def timer_end(self, timer, message):
        if self.config['options']['timer']:
            self.log(message + 'time: ' + str(time.time() - timer))

    def command_run(self, name, command):
        if self.config['options']['redirect_stdout']:
            path = os.path.join(self.dir_log, 'log_' + name + '.stdout')
            command += " 1> " + path
        if self.config['options']['redirect_stderr']:
            path = os.path.join(self.dir_log, 'log_' + name + '.stderr')
            command += " 2> " + path
        os.system(command)

    def stage(self, name):
        def decorator(func):
            def inner():
                self.log('start of stage ' + name)
                timer = self.timer_start()
                func()
                self.log('end of stage ' + name)
                self.timer_end(timer, 'taken ')
                print()
            return inner
        return decorator
